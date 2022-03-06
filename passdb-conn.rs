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
				buffer.push_str(",");
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

// main
#[tokio::main]
async fn main() -> std::io::Result<()>
{
	//
	Ok(())
}
