use std::collections::HashMap;

use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http;
use axum::Json;
use axum_auth::AuthBasic;
use chrono::NaiveTime;
use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;

use super::forbidden;
use super::internal_error;
use super::CreationDateFilter;
use super::Error;
use super::Filters;
use super::Pagination;
use super::Response;
use super::SortBySelector;
use super::Sorting;
use crate::db::ConnectionPool;
use crate::news::auth::authenticate;
use crate::news::auth::authorize_author;
use crate::news::auth::authorize_self;
use crate::news::auth::authorize_self_or_admin;
use crate::news::models::Category;
use crate::news::models::NewStory;
use crate::news::models::NewStorySerializer;
use crate::news::models::Story;
use crate::news::models::StoryNested;
use crate::news::models::Tag;
use crate::news::models::TagStory;
use crate::news::models::UpdatableStory;
use crate::news::models::UpdatableStorySerializer;
use crate::news::models::User;
use crate::schema::categories;
use crate::schema::stories;
use crate::schema::stories::dsl::*;
use crate::schema::tags;
use crate::schema::tags_stories;
use crate::schema::users;

// TODO query & nesting should happen inside one transaction
pub async fn get_stories(
    State(pool): State<ConnectionPool>,
    Query(pagination): Query<Pagination>,
    Query(filters): Query<Filters>,
    Query(sorting): Query<Sorting>,
) -> Result<Json<Vec<StoryNested>>, Error> {
    let pagination = pagination.configure();
    let mut conn = pool.get().map_err(internal_error)?;

    let mut news_sql = stories::table
        .filter(is_published.eq(true))
        .inner_join(users::table)
        .left_join(categories::table)
        .left_join(tags_stories::table.inner_join(tags::table))
        .order(stories::columns::id.asc())
        .offset(pagination.offset)
        .limit(pagination.limit)
        .into_boxed();

    if let Some(author_name) = filters.author_name {
        news_sql = news_sql.filter(users::columns::name.eq(author_name));
    }

    if let Some(cat_id) = filters.category_id {
        news_sql = news_sql.filter(categories::columns::id.eq(cat_id));
    }

    if let Some(creation_date) = filters.creation_date {
        let day_start = NaiveTime::from_num_seconds_from_midnight_opt(0, 0).unwrap();
        let day_end = NaiveTime::from_num_seconds_from_midnight_opt(86399, 1_999_999_999).unwrap();
        match creation_date {
            CreationDateFilter::CreationDateAt(date_at) => {
                let dt_lower = date_at.and_time(day_start);
                let dt_upper = date_at.and_time(day_end);

                news_sql = news_sql.filter(creation_timestamp.between(dt_lower, dt_upper))
            }
            CreationDateFilter::CreationDateUntil(date_at) => {
                let date_time = date_at.and_time(day_end);

                news_sql = news_sql.filter(creation_timestamp.le(date_time))
            }
            CreationDateFilter::CreationDateSince(date_at) => {
                let date_time = date_at.and_time(day_start);

                news_sql = news_sql.filter(creation_timestamp.ge(date_time))
            }
        }
    }

    if let Some(title_ilike) = filters.title_ilike {
        news_sql = news_sql.filter(title.ilike(title_ilike));
    }

    if let Some(content_ilike) = filters.content_ilike {
        news_sql = news_sql.filter(content.ilike(content_ilike));
    }

    if let Some(tag_in) = filters.tag_in {
        let tag_in: Vec<String> = tag_in.split(',').map(String::from).collect();

        news_sql = news_sql.filter(tags::columns::name.eq_any(tag_in));
    }

    if let Some(sorting) = sorting.sort_by {
        match sorting {
            SortBySelector::Author => news_sql = news_sql.order(users::columns::name.asc()),
            SortBySelector::Category => news_sql = news_sql.order(categories::columns::name.asc()),
            SortBySelector::CreationTimestampAsc => {
                news_sql = news_sql.order(creation_timestamp.asc())
            }
            SortBySelector::CreationTimestampDesc => {
                news_sql = news_sql.order(creation_timestamp.desc())
            }
        }
    }

    let news: Vec<(Story, User, Option<Category>)> = news_sql
        .select((
            Story::as_select(),
            User::as_select(),
            Option::<Category>::as_select(),
        ))
        .distinct()
        .load(&mut conn)
        .map_err(internal_error)?;

    let news: Vec<StoryNested> = nest_stories(&mut conn, news)?;

    Ok(news.into())
}

// TODO query & nesting should happen inside one transaction
pub async fn search_stories(
    State(pool): State<ConnectionPool>,
    Path(search_query): Path<String>,
    Query(pagination): Query<Pagination>,
    Query(sorting): Query<Sorting>,
) -> Result<Json<Vec<StoryNested>>, Error> {
    let pagination = pagination.configure();
    let mut conn = pool.get().map_err(internal_error)?;

    let mut news_sql = stories::table
        .filter(is_published.eq(true))
        .inner_join(users::table)
        .left_join(categories::table)
        .left_join(tags_stories::table.inner_join(tags::table))
        .filter(
            title
                .ilike(&search_query)
                .or(content.ilike(&search_query))
                .or(categories::columns::name.ilike(&search_query))
                .or(tags::columns::name.ilike(&search_query))
                .or(users::columns::name.ilike(&search_query)),
        )
        .order(stories::columns::id.asc())
        .offset(pagination.offset)
        .limit(pagination.limit)
        .into_boxed();

    if let Some(sorting) = sorting.sort_by {
        match sorting {
            SortBySelector::Author => news_sql = news_sql.order(users::columns::name.asc()),
            SortBySelector::Category => news_sql = news_sql.order(categories::columns::name.asc()),
            SortBySelector::CreationTimestampAsc => {
                news_sql = news_sql.order(creation_timestamp.asc())
            }
            SortBySelector::CreationTimestampDesc => {
                news_sql = news_sql.order(creation_timestamp.desc())
            }
        }
    }

    let news: Vec<(Story, User, Option<Category>)> = news_sql
        .select((
            Story::as_select(),
            User::as_select(),
            Option::<Category>::as_select(),
        ))
        .distinct()
        .load(&mut conn)
        .map_err(internal_error)?;

    let news: Vec<StoryNested> = nest_stories(&mut conn, news)?;

    Ok(news.into())
}

pub async fn get_story(
    State(pool): State<ConnectionPool>,
    claims: AuthBasic,
    Path(id_selector): Path<i32>,
) -> Result<Json<Story>, Error> {
    let mut conn = pool.get().map_err(internal_error)?;

    let actor = authenticate(claims, &mut conn).map_err(|e| e.into_error())?;

    let story: Option<Story> = stories::table
        .find(id_selector)
        .get_result(&mut conn)
        .optional()
        .map_err(internal_error)?;

    match story {
        None => return Err((http::StatusCode::NOT_FOUND, String::new())),
        Some(story) => {
            authorize_self(story.user_id, &actor).map_err(forbidden)?;

            Ok(story.into())
        }
    }
}

pub async fn create_story(
    State(pool): State<ConnectionPool>,
    claims: AuthBasic,
    story_ser: Json<NewStorySerializer>,
) -> Response {
    let mut conn = pool.get().map_err(internal_error)?;

    let actor = authenticate(claims, &mut conn).map_err(|e| e.into_error())?;
    authorize_author(&actor).map_err(forbidden)?;

    let story_ser: NewStorySerializer = story_ser.0;
    let tags: Vec<i32> = story_ser.tags.clone();
    let story: NewStory = story_ser.into_new_story(actor.id);

    let entry: Story = insert_into(stories::table)
        .values(&story)
        .get_result(&mut conn)
        .map_err(internal_error)?;

    let tag_relations: Vec<TagStory> = tags
        .into_iter()
        .map(|tag_id| TagStory {
            tag_id,
            story_id: entry.id,
        })
        .collect();

    insert_into(tags_stories::table)
        .values(&tag_relations)
        .execute(&mut conn)
        .map_err(internal_error)?;

    Ok(http::StatusCode::CREATED)
}

pub async fn publish_story(
    State(pool): State<ConnectionPool>,
    claims: AuthBasic,
    Path(id_selector): Path<i32>,
) -> Response {
    let mut conn = pool.get().map_err(internal_error)?;

    let actor = authenticate(claims, &mut conn).map_err(|e| e.into_error())?;

    let story: Option<Story> = stories::table
        .find(id_selector)
        .get_result(&mut conn)
        .optional()
        .map_err(internal_error)?;

    match story {
        None => return Ok(http::StatusCode::NOT_FOUND),
        Some(story) => {
            authorize_self(story.user_id, &actor).map_err(forbidden)?;

            update(&story)
                .set(is_published.eq(true))
                .execute(&mut conn)
                .map_err(internal_error)?;
        }
    };

    Ok(http::StatusCode::NO_CONTENT)
}

pub async fn update_story(
    State(pool): State<ConnectionPool>,
    claims: AuthBasic,
    Path(id_selector): Path<i32>,
    updated_story_ser: Json<UpdatableStorySerializer>,
) -> Response {
    let mut conn = pool.get().map_err(internal_error)?;

    let actor = authenticate(claims, &mut conn).map_err(|e| e.into_error())?;

    let updated_story_ser: UpdatableStorySerializer = updated_story_ser.0;
    let tags = updated_story_ser.tags.clone();
    let updated_story: UpdatableStory = updated_story_ser.into_updatable_story();

    let story: Option<Story> = stories::table
        .find(id_selector)
        .get_result(&mut conn)
        .optional()
        .map_err(internal_error)?;

    match story {
        None => return Ok(http::StatusCode::NOT_FOUND),
        Some(story) => {
            authorize_self(story.user_id, &actor).map_err(forbidden)?;

            update(&story)
                .set(updated_story)
                .execute(&mut conn)
                .map_err(internal_error)?;

            if !tags.is_empty() {
                diesel::delete(tags_stories::table.filter(tags_stories::story_id.eq(story.id)))
                    .execute(&mut conn)
                    .map_err(internal_error)?;

                let tag_relations: Vec<TagStory> = tags
                    .into_iter()
                    .map(|tag_id| TagStory {
                        tag_id,
                        story_id: story.id,
                    })
                    .collect();

                insert_into(tags_stories::table)
                    .values(&tag_relations)
                    .execute(&mut conn)
                    .map_err(internal_error)?;
            }
        }
    };

    Ok(http::StatusCode::NO_CONTENT)
}

pub async fn delete_story(
    State(pool): State<ConnectionPool>,
    claims: AuthBasic,
    Path(id_selector): Path<i32>,
) -> Response {
    let mut conn = pool.get().map_err(internal_error)?;

    let actor = authenticate(claims, &mut conn).map_err(|e| e.into_error())?;

    let story: Option<Story> = stories::table
        .find(id_selector)
        .get_result(&mut conn)
        .optional()
        .map_err(internal_error)?;

    match story {
        None => return Ok(http::StatusCode::NOT_FOUND),
        Some(story) => {
            authorize_self_or_admin(story.user_id, &actor).map_err(forbidden)?;

            delete(&story).execute(&mut conn).map_err(internal_error)?;
        }
    }

    Ok(http::StatusCode::NO_CONTENT)
}

fn nest_stories(
    conn: &mut PgConnection,
    news: Vec<(Story, User, Option<Category>)>,
) -> Result<Vec<StoryNested>, Error> {
    let cats: Vec<Category> = categories::table
        .order(categories::columns::id.asc())
        .load::<Category>(conn)
        .map_err(internal_error)?;
    let cats_dict: HashMap<i32, &Category> = cats.iter().map(|c| (c.id, c)).collect();

    let storys: Vec<&Story> = news.iter().map(|t| &t.0).collect();
    let tags: Vec<(TagStory, Tag)> = TagStory::belonging_to(&storys)
        .inner_join(tags::table)
        .select((TagStory::as_select(), Tag::as_select()))
        .load(conn)
        .map_err(internal_error)?;
    let news: Vec<(Story, User, Option<Category>, Vec<Tag>)> = tags
        .grouped_by(&storys)
        .into_iter()
        .zip(news)
        .map(|(tgs, (s, u, c))| (s, u, c, tgs.into_iter().map(|(_, t)| t).collect()))
        .collect();

    let news: Vec<StoryNested> = news
        .into_iter()
        .map(|t| StoryNested::from_tuple(t, &cats_dict))
        .collect();
    Ok(news)
}
