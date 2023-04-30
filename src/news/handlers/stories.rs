use axum::extract::Path;
use axum::extract::Query;
use axum::http;
use axum::Json;
use axum_auth::AuthBasic;
use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;

use super::forbidden;
use super::internal_error;
use super::Error;
use super::Pagination;
use super::Response;
use crate::db::establish_connection;
use crate::news::auth::authenticate;
use crate::news::auth::authorize_author;
use crate::news::auth::authorize_self;
use crate::news::auth::authorize_self_or_admin;
use crate::news::models::NewStory;
use crate::news::models::NewStorySerializer;
use crate::news::models::Story;
use crate::news::models::TagStory;
use crate::news::models::UpdatableStory;
use crate::news::models::UpdatableStorySerializer;
use crate::schema::stories;
use crate::schema::stories::dsl::*;
use crate::schema::tags_stories;

pub async fn get_stories(Query(pagination): Query<Pagination>) -> Result<Json<Vec<Story>>, Error> {
    let pagination = pagination.configure();
    let mut conn = establish_connection();

    let news = stories::table
        .filter(is_published.eq(true))
        .order(stories::columns::id.asc())
        .offset(pagination.offset)
        .limit(pagination.limit)
        .load::<Story>(&mut conn)
        .map_err(internal_error)?;

    Ok(news.into())
}

pub async fn get_story(
    claims: AuthBasic,
    Path(id_selector): Path<i32>,
) -> Result<Json<Story>, Error> {
    let actor = authenticate(claims).map_err(|e| e.into_error())?;

    let mut conn = establish_connection();

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

pub async fn create_story(claims: AuthBasic, story_ser: Json<NewStorySerializer>) -> Response {
    let actor = authenticate(claims).map_err(|e| e.into_error())?;
    authorize_author(&actor).map_err(forbidden)?;

    let story_ser: NewStorySerializer = story_ser.0;
    let tags: Vec<i32> = story_ser.tags.clone();
    let story: NewStory = story_ser.into_new_story(actor.id);
    let mut conn = establish_connection();

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

pub async fn publish_story(claims: AuthBasic, Path(id_selector): Path<i32>) -> Response {
    let actor = authenticate(claims).map_err(|e| e.into_error())?;

    let mut conn = establish_connection();

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
    claims: AuthBasic,
    Path(id_selector): Path<i32>,
    updated_story_ser: Json<UpdatableStorySerializer>,
) -> Response {
    let actor = authenticate(claims).map_err(|e| e.into_error())?;

    let updated_story_ser: UpdatableStorySerializer = updated_story_ser.0;
    let tags = updated_story_ser.tags.clone();
    let updated_story: UpdatableStory = updated_story_ser.into_updatable_story();

    let mut conn = establish_connection();

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

pub async fn delete_story(claims: AuthBasic, Path(id_selector): Path<i32>) -> Response {
    let actor = authenticate(claims).map_err(|e| e.into_error())?;

    let mut conn = establish_connection();

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
