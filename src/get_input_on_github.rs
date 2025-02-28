use octocrab::models::CommentId;
use octocrab::models::pulls::Comment;
 async fn run() -> octocrab::Result<Comment> {
    let octocrab = octocrab::Octocrab::default();
    let _ = octocrab.pulls("Jagoum", "fibbot").comment(CommentId(21)).delete();
    let _ = octocrab.pulls("Jagoum", "fibbot").comment(CommentId(42)).update("new comment");
    let comment = octocrab.pulls("Jagoum", "fibbot").comment(CommentId(42)).get().await;

    comment
 }