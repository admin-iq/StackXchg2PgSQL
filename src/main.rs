mod errors;
mod model;
mod schema;

use clap::Parser;
use diesel::prelude::*;
use model::{Badge, Comment, Post, PostHistory, PostLink, Tag, User, Vote, XmlModelIterator};

#[derive(Clone, Debug, Parser)]
#[command(version, about, long_about = None)]
/// A command-line tool to load StackExchange XML dumps into a
/// PostgreSQL database.
struct Arguments {
    /// The path to the badges XML file.
    #[arg(long)]
    badges_path: std::path::PathBuf,

    /// The path to the comments XML file.
    #[arg(long)]
    comments_path: std::path::PathBuf,

    /// The path to the posts XML file.
    #[arg(long)]
    posts_path: std::path::PathBuf,

    /// The path to the post history XML file.
    #[arg(long)]
    post_history_path: std::path::PathBuf,

    /// The path to the post links XML file.
    #[arg(long)]
    post_links_path: std::path::PathBuf,

    /// The path to the tags XML file.
    #[arg(long)]
    tags_path: std::path::PathBuf,

    /// The path to the users XML file.
    #[arg(long)]
    users_path: std::path::PathBuf,

    /// The path to the votes XML file.
    #[arg(long)]
    votes_path: std::path::PathBuf,

    /// The database URL.
    #[arg(long)]
    database_url: String,
}

/// Loads badge data from XML file into the database
///
/// # Arguments
///
/// * `connection` - A mutable reference to the PostgreSQL database connection
/// * `file_path` - Path to the badges XML file
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful, Error otherwise
///
/// # Errors
///
/// Returns an error if:
/// - XML file cannot be read or parsed
/// - Database insert operation fails
fn load_badges(
    connection: &mut PgConnection,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create iterator for Badge objects from XML file
    let badges_iter = XmlModelIterator::<Badge>::new(file_path)
        .map_err(|e| format!("Error creating XML model iterator for badges: {}", e))?;

    // Process each badge in the XML file
    for badge in badges_iter {
        match badge {
            Ok(badge) => {
                // Insert the badge into the database
                diesel::insert_into(schema::badges::table)
                    .values(&badge)
                    .execute(connection)
                    .map_err(|e| format!("Error inserting badge into database: {}", e))?;
            }
            Err(e) => eprintln!("Error processing badge: {}", e), // Log error but continue processing
        }
    }

    Ok(())
}

/// Loads comment data from XML file into the database
///
/// # Arguments
///
/// * `connection` - A mutable reference to the PostgreSQL database connection
/// * `file_path` - Path to the comments XML file
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful, Error otherwise
///
/// # Errors
///
/// Returns an error if:
/// - XML file cannot be read or parsed
/// - Database insert operation fails
fn load_comments(
    connection: &mut PgConnection,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create iterator for Comment objects from XML file
    let comments_iter = XmlModelIterator::<Comment>::new(file_path)
        .map_err(|e| format!("Error creating XML model iterator for comments: {}", e))?;

    // Process each comment in the XML file
    for comment in comments_iter {
        match comment {
            Ok(comment) => {
                // Insert the comment into the database
                diesel::insert_into(schema::comments::table)
                    .values(&comment)
                    .execute(connection)
                    .map_err(|e| format!("Error inserting comment into database: {}", e))?;
            }
            Err(e) => eprintln!("Error processing comment: {}", e), // Log error but continue processing
        }
    }

    Ok(())
}

/// Loads post data from XML file into the database
///
/// # Arguments
///
/// * `connection` - A mutable reference to the PostgreSQL database connection
/// * `file_path` - Path to the posts XML file
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful, Error otherwise
///
/// # Errors
///
/// Returns an error if:
/// - XML file cannot be read or parsed
/// - Database insert operation fails
fn load_posts(
    connection: &mut PgConnection,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create iterator for Post objects from XML file
    let posts_iter = XmlModelIterator::<Post>::new(file_path)
        .map_err(|e| format!("Error creating XML model iterator for posts: {}", e))?;

    // Process each post in the XML file
    for post in posts_iter {
        match post {
            Ok(post) => {
                // Insert the post into the database
                diesel::insert_into(schema::posts::table)
                    .values(&post)
                    .execute(connection)
                    .map_err(|e| format!("Error inserting post into database: {}", e))?;
            }
            Err(e) => eprintln!("Error processing post: {}", e), // Log error but continue processing
        }
    }

    Ok(())
}

/// Loads post history data from XML file into the database
///
/// # Arguments
///
/// * `connection` - A mutable reference to the PostgreSQL database connection
/// * `file_path` - Path to the post history XML file
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful, Error otherwise
///
/// # Errors
///
/// Returns an error if:
/// - XML file cannot be read or parsed
/// - Database insert operation fails
fn load_post_history(
    connection: &mut PgConnection,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create iterator for PostHistory objects from XML file
    let post_history_iter = XmlModelIterator::<PostHistory>::new(file_path)
        .map_err(|e| format!("Error creating XML model iterator for post history: {}", e))?;

    // Process each post history record in the XML file
    for post_history in post_history_iter {
        match post_history {
            Ok(post_history) => {
                // Insert the post history record into the database
                diesel::insert_into(schema::post_historys::table)
                    .values(&post_history)
                    .execute(connection)
                    .map_err(|e| format!("Error inserting post history into database: {}", e))?;
            }
            Err(e) => eprintln!("Error processing post history: {}", e), // Log error but continue processing
        }
    }

    Ok(())
}

/// Loads post links data from XML file into the database
///
/// # Arguments
///
/// * `connection` - A mutable reference to the PostgreSQL database connection
/// * `file_path` - Path to the post links XML file
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful, Error otherwise
///
/// # Errors
///
/// Returns an error if:
/// - XML file cannot be read or parsed
/// - Database insert operation fails
fn load_post_links(
    connection: &mut PgConnection,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create iterator for PostLink objects from XML file
    let post_links_iter = XmlModelIterator::<PostLink>::new(file_path)
        .map_err(|e| format!("Error creating XML model iterator for post links: {}", e))?;

    // Process each post link in the XML file
    for post_link in post_links_iter {
        match post_link {
            Ok(post_link) => {
                // Insert the post link into the database
                diesel::insert_into(schema::post_links::table)
                    .values(&post_link)
                    .execute(connection)
                    .map_err(|e| format!("Error inserting post link into database: {}", e))?;
            }
            Err(e) => eprintln!("Error processing post link: {}", e), // Log error but continue processing
        }
    }

    Ok(())
}

/// Loads tag data from XML file into the database
///
/// # Arguments
///
/// * `connection` - A mutable reference to the PostgreSQL database connection
/// * `file_path` - Path to the tags XML file
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful, Error otherwise
///
/// # Errors
///
/// Returns an error if:
/// - XML file cannot be read or parsed
/// - Database insert operation fails
fn load_tags(
    connection: &mut PgConnection,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create iterator for Tag objects from XML file
    let tags_iter = XmlModelIterator::<Tag>::new(file_path)
        .map_err(|e| format!("Error creating XML model iterator for tags: {}", e))?;

    // Process each tag in the XML file
    for tag in tags_iter {
        match tag {
            Ok(tag) => {
                // Insert the tag into the database
                diesel::insert_into(schema::tags::table)
                    .values(&tag)
                    .execute(connection)
                    .map_err(|e| format!("Error inserting tag into database: {}", e))?;
            }
            Err(e) => eprintln!("Error processing tag: {}", e), // Log error but continue processing
        }
    }

    Ok(())
}

/// Loads user data from XML file into the database
///
/// # Arguments
///
/// * `connection` - A mutable reference to the PostgreSQL database connection
/// * `file_path` - Path to the users XML file
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful, Error otherwise
///
/// # Errors
///
/// Returns an error if:
/// - XML file cannot be read or parsed
/// - Database insert operation fails
fn load_users(
    connection: &mut PgConnection,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create iterator for User objects from XML file
    let users_iter = XmlModelIterator::<User>::new(file_path)
        .map_err(|e| format!("Error creating XML model iterator for users: {}", e))?;

    // Process each user in the XML file
    for user in users_iter {
        match user {
            Ok(user) => {
                // Insert the user into the database
                diesel::insert_into(schema::users::table)
                    .values(&user)
                    .execute(connection)
                    .map_err(|e| format!("Error inserting user into database: {}", e))?;
            }
            Err(e) => eprintln!("Error processing user: {}", e), // Log error but continue processing
        }
    }

    Ok(())
}

/// Loads vote data from XML file into the database
///
/// # Arguments
///
/// * `connection` - A mutable reference to the PostgreSQL database connection
/// * `file_path` - Path to the votes XML file
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful, Error otherwise
///
/// # Errors
///
/// Returns an error if:
/// - XML file cannot be read or parsed
/// - Database insert operation fails
fn load_votes(
    connection: &mut PgConnection,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create iterator for Vote objects from XML file
    let votes_iter = XmlModelIterator::<Vote>::new(file_path)
        .map_err(|e| format!("Error creating XML model iterator for votes: {}", e))?;

    // Process each vote in the XML file
    for vote in votes_iter {
        match vote {
            Ok(vote) => {
                // Insert the vote into the database
                diesel::insert_into(schema::votes::table)
                    .values(&vote)
                    .execute(connection)
                    .map_err(|e| format!("Error inserting vote into database: {}", e))?;
            }
            Err(e) => eprintln!("Error processing vote: {}", e), // Log error but continue processing
        }
    }

    Ok(())
}

fn main() {
    // Parse the command-line arguments.
    let args: Arguments = Arguments::parse();

    // Connect to the PostgreSQL database.
    let database_url = args.database_url.to_string();

    let mut connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    connection
        .transaction(|connection| {
            // Load the badges XML file into the database.
            let badges_path = args
                .badges_path
                .to_str()
                .expect("Please provide a valid badges file path.");

            load_badges(connection, badges_path)
                .unwrap_or_else(|e| panic!("Failed to load badges: {}", e));

            // Load the comments XML file into the database.
            let comments_path = args
                .comments_path
                .to_str()
                .expect("Please provide a valid comments file path.");

            load_comments(connection, comments_path)
                .unwrap_or_else(|e| panic!("Failed to load comments: {}", e));

            // Load the posts XML file into the database.
            let posts_path = args
                .posts_path
                .to_str()
                .expect("Please provide a valid posts file path.");

            load_posts(connection, posts_path)
                .unwrap_or_else(|e| panic!("Failed to load posts: {}", e));

            // Load the post history XML file into the database.
            let post_history_path = args
                .post_history_path
                .to_str()
                .expect("Please provide a valid post history file path.");

            load_post_history(connection, post_history_path)
                .unwrap_or_else(|e| panic!("Failed to load post history: {}", e));

            // Load the post links XML file into the database.
            let post_links_path = args
                .post_links_path
                .to_str()
                .expect("Please provide a valid post links file path.");

            load_post_links(connection, post_links_path)
                .unwrap_or_else(|e| panic!("Failed to load post links: {}", e));

            // Load the tags XML file into the database.
            let tags_path = args
                .tags_path
                .to_str()
                .expect("Please provide a valid tags file path.");

            load_tags(connection, tags_path)
                .unwrap_or_else(|e| panic!("Failed to load tags: {}", e));

            // Load the users XML file into the database.
            let users_path = args
                .users_path
                .to_str()
                .expect("Please provide a valid users file path.");

            load_users(connection, users_path)
                .unwrap_or_else(|e| panic!("Failed to load users: {}", e));

            // Load the votes XML file into the database.
            let votes_path = args
                .votes_path
                .to_str()
                .expect("Please provide a valid votes file path.");

            load_votes(connection, votes_path)
                .unwrap_or_else(|e| panic!("Failed to load votes: {}", e));

            // Commit the transaction.
            diesel::result::QueryResult::Ok(())
        })
        .unwrap_or_else(|_| panic!("Error loading data into the database"));

    println!("Data loaded successfully into the database.");
}
