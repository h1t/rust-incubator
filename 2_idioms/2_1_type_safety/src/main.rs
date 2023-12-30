use step_2_1::{
    post::{Body, Id as PostId, Title},
    user::Id as UserId,
    New, Post,
};

fn main() {
    let new_post = Post::<New>::new(
        PostId(5),
        UserId(1),
        Title("title".to_owned()),
        Body("body".to_owned()),
    );

    let unmoderated = new_post.publish();
    let unmoderated_1 = unmoderated.clone();

    let published = unmoderated.allow();

    let _deleted = published.delete();
    let _deleted_1 = unmoderated_1.deny();
}
