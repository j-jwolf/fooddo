use std::io::{Write};
use std::fs::{File, read_to_string, remove_file};
use aws_config::meta::region::{RegionProviderChain};
use aws_sdk_dynamodb::{Client, Error, Region, PKG_VERSION};
use aws_sdk_dynamodb::model::{AttributeValue};
use aws_sdk_dynamodb::client::fluent_builders::Query;
use aws_sdk_dynamodb::output::QueryOutput;
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::error::QueryError;
use std::process;
use tokio_stream::StreamExt;

/*
debug function -- prints type of argument passed in as reference

call: print_type(&your_variable)
*/
fn print_type<T>(_: &T) {println!("{}", std::any::type_name::<T>());}

/*
debug function -- reads contents of file and returns as string

returns contents of file on success else "file read error"
*/
fn read_file(filename: &str) -> String {return read_to_string(filename).expect("file read error");}

/*
writes string (data) to file (filename)

returns true if success else false
*/
fn write_file(data: &str, filename: &str) -> bool
{
	let mut file = File::create(filename).unwrap();
	let res = writeln!(&mut file, "{}", data);
	match res
	{
		Ok(_) => return true,
		Err(e) => {
			eprintln!("Error while writing to {}:\n{}", filename, e);
			return false;
		},
	}
}

/*
lists tables stored with keys stored in .aws/config

return empty result on success else error
*/
async fn list_tables(client: &Client) -> std::result::Result<(), Error>
{
	let tables = client.list_tables().send().await?;
	println!("Current tables:\n{:?}", tables);
	Ok(())
}

/*
add password object to table

return empty result on success else error
*/
async fn add_item(
	client: &Client,
	table: &str,
	user: &str,
	password: &str,
	email: &str
) -> std::result::Result<(), Error>
{
	let user_av = AttributeValue::S(user.into());
	let password_av = AttributeValue::S(password.into());
	let email_av = AttributeValue::S(email.into());
	let request = client
		.put_item()
		.table_name(table)
		.item("user", user_av)
		.item("password", password_av)
		.item("email", email_av);
	println!("Executing request {:?} to add item to {}", request, table);
	request.send().await?;
	println!("Added user {} with password {} and email {} to table", user, password, email);
	Ok(())
}

/*
debug -- lists all items in a table

!!! NOTE: USE SPARINGLY, THIS IS VERY EXPENSIVE!!!

returns empty result on success else error
*/
async fn test_scan(client: &Client, table: &str) -> std::result::Result<(), Error>
{
	let items: Result<Vec<_>, _> = client
		.scan()
		.table_name(table)
		.into_paginator()
		.items()
		.send()
		.collect()
		.await;
	println!("Items in table:");
	for item in items? {println!("	{:?}", item);}
	Ok(())
}

/*
debug -- adds test value to test table

FOR REFERENCE TO UNDERSTAND CODE ONLY

adds SongTitle by Artist to test table Musician
*/
async fn test(
	client: &Client,
	table: &str,
	artist: &str,
	song: &str
) -> std::result::Result<(), Error>
{
	println!("In test()");
	let art_av = AttributeValue::S(artist.into());
	let song_av = AttributeValue::S(song.into());
	let request = client
		.put_item()
		.table_name(table)
		.item("Artist", art_av)
		.item("SongTitle", song_av);
	//println!("Executing request {:?} to add item to {}", request, table);
	request.send().await?;
	println!("Added artist {} with song {} to {}", artist, song, table);
	Ok(())
}

/*
deletes an item from table

!!! DOES NOT WORK IN CURRENT BUILD -- YOU CAN CALL IT BUT IT WILL NOT WORK !!!

returns empty result on success else error
*/
async fn delete_item(client: &Client, table: &str, key: &str, value: &str) -> std::result::Result<(), Error>
{
	match client
		.delete_item()
		.table_name(table)
		.key(key, AttributeValue::S(value.into()))
		.send()
		.await
	{
		Ok(_) => println!("Deleted"),
		Err(e) => {
			eprintln!("Error:\n{}", e);
			process::exit(-1);
		},
	}
	Ok(())
}

/*
debug -- queries test table musician for SongTitles by Artist

returns QueryOutput structure
*/
async fn test_query(client: &Client, table: &str, artist: &str) -> QueryOutput
{
	println!("in test_query()");
	let req = client
		.query()
		.table_name(table)
		.key_condition_expression("Artist = :artist")
		.expression_attribute_values(":artist", AttributeValue::S(artist.to_string()))
		.send()
		.await;
	match req
	{
		Ok(_) => return req.unwrap(),
		Err(e) => {
			eprintln!("{}", e);
			process::exit(-1);
		},
	}
	//dbg!(req);
	//println!("debug test_query: {:?}", dbg!(req));
}

// main function
#[tokio::main]
async fn main() -> std::result::Result<(), Error>
{
	let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"))
		.or_default_provider()
		.or_else(Region::new("us-east-1"));
	println!();
	let shared_config = aws_config::from_env().region(region_provider).load().await;
	let client = Client::new(&shared_config);
	list_tables(&client).await?;
	test(
		&client,
		"Musician",
		"No One You Know",
		"Call Me Today"
	).await?;
	test(
		&client,
		"Musician",
		"No One You Know",
		"My Dog Spot"
	).await?;
	test(
		&client,
		"Musician",
		"No One You Know",
		"Somewhere Down The Road"
	).await?;
	test(
		&client,
		"Musician",
		"The Acme Band",
		"Still in Love"
	).await?;
	test(
		&client,
		"Musician",
		"The Acme Band",
		"Look Out, World"
	).await?;
	//test_scan(&client, "Musician").await?;
	let q = test_query(
		&client,
		"Musician",
		"The Acme Band"
	);
	let items = q.await.items.unwrap();
	println!("{:?}", items);
	let mut buffer: String = "".to_string().to_owned();
	for item in items
	{
		buffer.push_str(item["SongTitle"].as_s().unwrap());
		buffer.push_str("\n");
	}
	println!("buffer:\n{}", buffer);
	Ok(())
}
