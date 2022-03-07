use std::env;
use std::io::{Write};
use std::fs::{File, read_to_string, remove_file};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error, Region, PKG_VERSION};
use aws_sdk_dynamodb::model::{AttributeValue};
use aws_sdk_dynamodb::client::fluent_builders::{Query};
use aws_sdk_dynamodb::output::QueryOutput;
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_dynamodb::error::QueryError;
use std::process;
use tokio_stream::StreamExt;

/* DEBUG FUNCTIONS */

/*
prints type of variable passed in via reference

call: print_type(&your_variable)
*/
fn print_type<T>(_: &T) {println!("{}", std::any::type_name::<T>());}

/*
!!! THIS IS VERY EXPENSIVE -- USE SPARINGLY AND ONLY WHEN NECCESSARY FOR DEBUGGING !!!

lists all items in a table

return:
	success: None
	error: Error
*/
async fn scan_table(client: &Client, table: &str) -> std::result::Result<(), Error>
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
list tables client has access to

return:
	success: empty
	error: Error
*/
async fn list_tables(client: &Client) -> std::result::Result<(), Error>
{
	let tables = client.list_tables().send().await?;
	println!("Current tables:\n{:?}", tables);
	Ok(())
}

/* UTILITIES */

/*
reads contents of file filename into string

return:
	success: contents of file
	error: "file read error"
*/
fn read_file(filename: &str) -> String {return read_to_string(filename).expect("file read error");}

/*
writes data to file filename

return:
	success: true
	error: false
*/
fn write_file(data: &str, filename: &str) -> bool
{
	let mut file = File::create(filename).unwrap();
	let res = writeln!(&mut file, "{}", data);
	match res
	{
		Ok(_) => return true,
		Err(e) => {
			eprintln!("Error writing to file {}:\n{}", filename, e);
			return false;
		},
	}
}

/*
notifies user that arguments do not match function call and exits program

EXIT CODE: -3
*/
fn arg_error(actual: usize, expected: i32)
{
	eprintln!("Argument error -- expected {} arguments and got {}", expected, actual);
	process::exit(-3);
}

/* DATABASE FUNCTIONS */

/*
add user to User table

return:
	success: empty
	error: Error
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
	let pass_av = AttributeValue::S(password.into());
	let email_av = AttributeValue::S(email.into());
	let request = client
		.put_item()
		.table_name(table)
		.item("User", user_av)
		.item("Password", pass_av)
		.item("Email", email_av);
	println!("Executing request {:?} to add item to {}", request, table);
	request.send().await?;
	println!("Added user {} to {}", user, table);
	Ok(())
}

/*
!!! DOES NOT WORK IN CURRENT BUILD -- DO NOT CALL !!!

deletes item from table

return:
	success: empty
	error: Error

EXIT CODE: -1
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
			eprintln!("Error deleting item from {}:\n{}", table, e);
			process::exit(-1);
		},
	}
	Ok(())
}

/*
query users by email and write attributes to query.csv

if email not found, will write \n to file

return:
	success: true
	error: false
*/
async fn get_user_by_email(client: &Client, table: &str, email: &str) -> bool
{
	let request = client
		.query()
		.table_name(table)
		.key_condition_expression("Email = :email")
		.expression_attribute_values(":email", AttributeValue::S(email.to_string()))
		.send()
		.await;
	match request
	{
		Ok(_) => {
			let items = request.unwrap().items.unwrap();
			let mut buffer: String = "".to_string().to_owned();
			for item in items
			{
				buffer.push_str(item["Email"].as_s().unwrap());
				buffer.push_str(",");
				buffer.push_str(item["User"].as_s().unwrap());
				buffer.push_str(",");
				buffer.push_str(item["Password"].as_s().unwrap());
			}
			write_file(&buffer, "query.csv");
			return true;
		},
		Err(e) => {
			eprintln!("{}", e);
			return false;
		},
	}
}

/*
main

EXIT CODE: -2, -4
*/
#[tokio::main]
async fn main() -> std::io::Result<()>
{
	let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"))
		.or_default_provider()
		.or_else(Region::new("us-east-1")); // this is a temp fix -- !!! FIX ME !!!
	println!();
	let shared_config = aws_config::from_env().region(region_provider).load().await;
	let client = Client::new(&shared_config);
	/*
	args coming in:
		[0]: function (add, delete, get, list, scan)
		[1:n-1]: args for function
	*/
	let argv: Vec<String> = env::args().collect();
	let argc = argv.len();
	if argc == 1
	{
		eprintln!("No arguments provided");
		process::exit(-2);
	}
	else if argc == 2 && argv[1] == "h"
	{
		println!(
			"{}",
			read_file("info.fdo")
		);
		process::exit(0);
	}
	let table = "User";
	let fun = &argv[1];
	if fun == "add"
	{
		if argc < 5 {arg_error(argc-1, 4);}
		let user = &argv[2];
		let password = &argv[3];
		let email = &argv[4];
		add_item(&client, table, user, password, email).await;
	}
	else if fun == "delete"
	{
		if argc < 4 {arg_error(argc-1, 3);}
		let key = &argv[2];
		let value = &argv[3];
		delete_item(&client, table, key, value).await;
	}
	else if fun == "get"
	{
		if argc < 3 {arg_error(argc-1, 2);}
		let email = &argv[2];
		get_user_by_email(&client, table, email).await;
	}
	else if fun == "list" {list_tables(&client).await;}
	else if fun == "scan" {scan_table(&client, table).await;}
	else if fun == "dquery"
	{
		let filename = "query.csv";
		if std::path::Path::new(filename).exists() {remove_file(filename);}
	}
	else
	{
		eprintln!("{} is not a valid function. cargo run h for information", fun); // not implemented yet
		process::exit(-4);
	}
	Ok(())
}
