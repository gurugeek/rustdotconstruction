// Copyright (c) 2016 The Rouille developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate postgres;
#[macro_use]
extern crate rouille;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate horrorshow;

extern crate rust_tags;

//use rust_tags::core::*;
use rust_tags::tags::*;
use rust_tags::tags::title;
use rust_tags::attributes::*;

use std::sync::Mutex;
use horrorshow::prelude::*;
use horrorshow::helper::doctype;

use postgres::Connection;
use postgres::TlsMode;
use postgres::transaction::Transaction;

use rouille::Request;
use rouille::Response;

fn main() {
    // This example demonstrates how to connect to a database and perform queries when the client
    // performs a request.
    // The server created in this example uses a REST API.

    // The first thing we do is try to connect to the database.
    //
    // One important thing to note here is that we wrap a `Mutex` around the connection. Since the
    // request handler can be called multiple times in parallel, everything that we use in it must
    // be thread-safe. By default the PostgresSQL connection isn't thread-safe, so we need a mutex
    // to make it thread-safe.
    //
    // Not wrapping a mutex around the database would lead to a compilation error when we attempt
    // to use the variable `db` from within the closure passed to `start_server`.
    let db = {
        let db = Connection::connect(
            "postgres://postgres:123@localhost/doctorstrange",
            TlsMode::None,
        );
        Mutex::new(db.expect("Failed to connect to database"))
    };

    // We perform some initialization for the sake of the example.
    // In a real application you probably want to have a migrations system. This is out of scope
    // of rouille.
    {
        let sql = "CREATE TABLE IF NOT EXISTS notes (
                    id SERIAL PRIMARY KEY,
                    content TEXT NOT NULL
                   );";
        db.lock()
            .unwrap()
            .execute(sql, &[])
            .expect("Failed to initialize database");
    }

    // Small message so that people don't need to read the source code.
    // Note that like all the other examples, we only listen on `localhost`, so you can't access this server
    // from any machine other than your own.
    println!("Now listening on localhost:8000");

    // Now the server starts listening. The `move` keyword will ensure that we move the `db` variable
    // into the closure. Not putting `move` here would result in a compilation error.
    //
    // Note that in an ideal world, `move` wouldn't be necessary here. Unfortunately Rust isn't
    // smart enough yet to understand that the database can't be destroyed while we still use it.
    rouille::start_server("0.0.0.0:8000", move |request| {
        // Since we wrapped the database connection around a `Mutex`, we lock it here before usage.
        //
        // This will give us exclusive access to the database connection for the handling of this
        // request. Unfortunately the consequence is that if a request is made while another one
        // is already being processed, the second one will have to wait for the first one to
        // complete.
        //
        // In a real application you probably want to create multiple connections instead of just
        // one, and make each request use a different connection.
        //
        // In addition to this, if a panic happens while the `Mutex` is locked then the database
        // connection will likely be in a corrupted state and the next time the mutex is locked
        // it will panic. This is another good reason to use multiple connections.
        let db = db.lock().unwrap();

        // Start a transaction so that if a panic happens during the processing of the request,
        // any change made to the database will be rolled back.
        let db = db.transaction().unwrap();

        // For better readability, we handle the request in a separate function.
        let response = note_routes(&request, &db);

        // If the response is a success, we commit the transaction before returning. It's only at
        // this point that data are actually written in the database.
        if response.is_success() {
            db.commit().unwrap();
        }

        response
    });
}

// This function actually handles the request.
fn note_routes(request: &Request, db: &Transaction) -> Response {
    router!(request,
            // (GET) (/) => {
            // For the sake of the example we just put a dummy route for `/` so that you see
            // something if you connect to the server with a browser.
            //    Response::text("Welcome to your first app :)")
            //},
        
        
            (GET) (/t) => {
        	
        	    let page_title = "Notes App";
        	    let mytext = "Note App I am a variable mytext";
        	    let batman = "Gotham";
        	    let superman = "Superman will arrive shortly";
        	    
                //Response::text("Welcome to your first app :)")
                let actual = format!("{}", html! {
                    : doctype::HTML;
                    html {
                        head {
                            title : page_title;
                            link(rel="stylesheet", type="text/css", href="https://cloud.typography.com/7964312/7143592/css/fonts.css");
                        }
                        body {
                            // attributes
                            
                            h1(id="heading") {
                	            div (style= "font-family: Giant Background A, Giant Background B;font-style: normal;font-weight: 400;font-size:30pt;")

                                // Insert escaped text
                                    : "Welcome to the NOTES.CONSTRUCTION App";                                
                            }
                            
                            //style (font-family="Tugsten A;")
                            div (style= "font-family: Gotham A, Gotham B;font-style: normal;font-weight: 400;"){
                                // Insert raw text = (unescaped)                               
                                : batman;
                                br; 
                                : "Batman is here";
                                br;
                                : superman;

                                form (action="/note", method="POST", enctype="text/plain");                   
                                input(type="text" ,name="Title", placeholder="Title");
                                br;
                                
                                textarea (name="abcd", placeholder="your note here", cols="40", rows= "5"){}
                                
                                //:Raw("</textarea>");
                                br;
                                input (type="submit");
                                
                                //p (input type="submit")                                
                                //: Raw("<i>test raw/i> !")
                            }
                            
                            div (style= "font-family: Tungsten A, Tungsten B;font-style: normal;font-weight: 400;font-size:30pt;") {
                                // Insert raw text = (unescaped)                                
                                : superman;                                
                            }
                            
                            // a link
                            a  (href="/notes") {                                
                                : "A list of Notes";
                            }

                            h2 (id="test") {
                                // Insert raw text (unescaped)
                                : mytext;
                            }
                            
                            ol(id="count") {
                                // You can embed for loops, while loops, and if statements.
                                @ for i in 0..10 {
                                    li(first? = (i == 0)) {
                                        // Format some text.
                                        : format_args!("{}", i+1)
                                    }
                                }
                            }
                            // You need semi-colons for tags without children.
                            br; br;

                            p {
                                // You can also embed closures.
                                |tmpl| {
                                    tmpl << "Easy!";
                                }
                            }
                        }
                    }
                });                
                
                rouille::Response::html (actual)
            },
            
            
            (GET) (/) => {
                let page_title = "Welcome to the NOTES.CONSTRUCTION App";
        	    let batman = "Fill the form to save your note";
        	    let superman = "by gurugeek | coded entirely in Rust -18/2/2018";
        	    
                //Response::text("Welcome to your first app :)")
                let actual = format!("{}", html! {
                    : doctype::HTML;
                    html {
                        head {
                            title : page_title;
                            link(rel="stylesheet", type="text/css", href="https://cloud.typography.com/7964312/7143592/css/fonts.css");
                        }
                        body {
                            // attributes
                            
                            h1(id="heading") {
                	            div (style= "font-family: Giant Background A, Giant Background B;font-style: normal;font-weight: 400;font-size:30pt;")
                                    : page_title;                   
                            }                            

                            //style (font-family="Tugsten A;")
                            div (style= "font-family: Gotham A, Gotham B;font-style: normal;font-weight: 400;") {
                                batman;
                                br; 
                                
                                form (action="/note", method="POST", enctype="text/plain");                   
                                input(type="text" ,name="Title", placeholder="Title");
                                br;
                                
                                textarea (name="abcd", placeholder="your note here", cols="40", rows= "5"){}

                                br;
                                input (type="submit");
                            }
                            
                            div (style= "font-family: Tungsten A, Tungsten B;font-style: normal;font-weight: 400;font-size:30pt;") {                                
                                : superman;
                            }
                            
                            // a link
                            a  (href="/notes") {
                                
                                : "A list of Notes";
                            }
                            
                            br;
                            br;                                                        
                        }
                    }
                });
                
                rouille::Response::html(actual)

            },
                
            //POST) (/submit) => {
            // This is the route that is called when the user submits the form of the
            // home page.

            // We query the data with the `post_input!` macro. Each field of the macro
            // corresponds to an element of the form.
            // If the macro returns an error (for example if a field is missing, which
            // can happen if you screw up the form or if the user made a manual request)
            // we return a 400 response. 
            //  let data = try_or_400!(post_input!(request, {
            //     txt: String,
            //    files: Vec<rouille::input::post::BufferedFile>,
            //}));

            // We just print what was received on stdout. Of course in a real application
            // you probably want to process the data, eg. store it in a database. 
            //  println!("Received data: {:?}", data);

            //  rouille::Response::html("Success! <a href=\"/\">Go back</a>.")
            //  },
            // }
            
            (GET) (/tags) => { //using rust tags :) 
 	            
 	            let superman = "superman";
 	            let notes = "Notes.Construction";
 	            let joker = "why so serious";
 	            
 	            
 	            let frag = html(&[
                    head(&[title(&["My Blog".into()])]),
                    link(&[rel("stylesheet"), _type("text/css"), href("https://cloud.typography.com/7964312/7143592/css/fonts.css")]),

                    body(&[
                        div(&[
                            //"Jason Longshore".into(),
                            notes.into(),
                            
                            br(),
                            br(),
                            rust_tags::attributes::style("font-family:Tungsten A, Tungsten B;font-style: normal;font-weight: 400;font-size:30pt;"),
                            
                            rust_tags::tags::form(&[action("/note"), method("POST"), enctype("text/plain"),
                                                    input(&[_type("text"), name("title"), placeholder("Some text")]),
                                                    input(&[_type("submit")]),
                                                    textarea(&[name("notes"), cols("40"), rows("40")])
                            ]),
                            superman.into(),
                            br(),
                            
                            //input(type="text" ,name="Title", placeholder="Some text");
                            
                            rust_tags::tags::form(&[id("my-form"), "this is my <form>".into()]),
                            //rust_tags::tags::textarea(&[name("notes"), cols ("40"), rows ("5").into()]),

                            hr(&[]),
                            
                            div(&[ //div to change font to gotham
                                
                                rust_tags::attributes::style("font-family:Gotham A, Gotham B;font-style: normal;font-weight: 400;font-size:30pt;"),
                                
                                joker.into(),
                                br(),

                                a(&[href("#"), "My Blog <hello world />".into()]),

                                br(),
                                br()
                            ]) //closed div 
                                
                        ])
                    ])
                ]); 	
                
                // When viewing the home page, we return an HTML document described below.
                rouille::Response::html(frag.data)
            },


            (GET) (/notes) => {
                
                let mut notes: Vec<i32> = vec![];

                for row in &db.query("SELECT id FROM notes", &[]).unwrap() {
                    let id: i32 = Some(row.get(0)).unwrap();
                    notes.push(id as i32);
                }
                
                let page_title = "notes";
                
                let actual = format!("{}", html! {
                    : doctype::HTML;
                    html {
                        head {
                            title : page_title;
                            link(rel="stylesheet", type="text/css", href="https://cloud.typography.com/7964312/7143592/css/fonts.css");
                        }
                        body {
                            // attributes
                            
                            h1(id="heading") {
                	            div (style= "font-family: Giant Background A, Giant Background B;font-style: normal;font-weight: 400;font-size:30pt;")
                                    : page_title;                   
                            }
                            
                            //style (font-family="Tugsten A;")
                            div (style= "font-family: Gotham A, Gotham B;font-style: normal;font-weight: 400;"){
                                br;                                 
                                
                                ul(id="notes_list") {
                                    @ for i in notes.iter() {
                                        li(id=format!("note_{}", i)) {
                                            a(href=format!("/note/{}", i)) {
                                                : format_args!("/note/{}", i)
                                            }
                                        }
                                    }
                                }

                                br; br;
                            }
                        }
                    }
                });
                
                rouille::Response::html(actual)
        },

        (GET) (/note/{id: i32}) => {
            // This route returns the content of a note, if it exists.
            // Note that this code is a bit unergonomic, but this is mostly a problem with the
            // database client library and not rouille itself.
            // To do so, we first create a variable that will receive the content of the note.
            let mut content: Option<String> = None;
            // And then perform the query and write to `content`. This line can only panic if the
            // SQL is malformed.
            for row in &db.query("SELECT content FROM notes WHERE id = $1", &[&id]).unwrap() {
                content = Some(row.get(0));
            }

            // If `content` is still empty at this point, this means that the note doesn't
            // exist in the database. Otherwise, we return the content.
           
           let page_title = "wow";
 
            let actual = html! {
                : doctype::HTML;
                html {
                    head {
                        title : page_title;
                        link(rel="stylesheet", type="text/css", href="https://cloud.typography.com/7964312/7143592/css/fonts.css");
                    }
                    body {
                        // attributes
                        h1(style= "font-family: Tungsten A, Tungsten B;font-style: normal;font-weight: 400;font-size:30pt;") {
                            // Insert escaped text
                            : "This is your note"                                
                        }

                        //style (font-family="Tugsten A;")
                        p (style= "font-family: Gotham A, Gotham B;font-style: normal;font-weight: 400;"){
                            // Insert raw text = (unescaped)                            
                            : &content;
                            //: Raw("<i>test raw/i> !")                            
                        } } }        
            }.into_string().unwrap();                        
            
            match content {            	
                Some(_content) => Response::html(actual),
                None => Response::empty_404(),
            }
        },


            (GET) (/notetag/{id: i32}) => {
                // This route returns the content of a note, if it exists.
                // Note that this code is a bit unergonomic, but this is mostly a problem with the
                // database client library and not rouille itself.
                // To do so, we first create a variable that will receive the content of the note.
                let mut content: Option<String> = None;
                // And then perform the query and write to `content`. This line can only panic if the
                // SQL is malformed.
                for row in &db.query("SELECT content FROM notes WHERE id = $1", &[&id]).unwrap() {
                    content = Some(row.get(0));                                        
                }

                // let superman = "superman";
   
 
 	            let notes = "Notes.Construction";
 	            let joker = "why so serious";
                //let notetag = content;

                let mut content_cloned = content.clone();
                
 	            let frag = html(&[
                    head(&[title(&["My Blog".into()])]),
                    link(&[rel("stylesheet"), _type("text/css"), href("https://cloud.typography.com/7964312/7143592/css/fonts.css")]),

                    body(&[
                        div(&[
                            //"Jason Longshore".into(),
                            notes.into(),
                            
                            
                            br(),br(),
                            rust_tags::attributes::style("font-family:Tungsten A, Tungsten B;font-style: normal;font-weight: 400;font-size:30pt;"),
                            
                            rust_tags::tags::form(&[action("/note"), method("POST"), enctype("text/plain"),


                                                    input(&[_type("text"), name("title"), placeholder("Some text")]),
                                                    input(&[_type("submit")]),
                                                    textarea(&[name("notes"), cols("40"), rows("40")])
                            ]),
                
                            br(),
                
                            //input(type="text" ,name="Title", placeholder="Some text");
                            //rust_tags::tags::textarea(&[name("notes"), cols ("40"), rows ("5").into()]),
                            hr(&[]),
                            
                            div(&[ //div to change font to gotham
            
                                rust_tags::attributes::style("font-family:Gotham A, Gotham B;font-style: normal;font-weight: 400;font-size:30pt;"),
                
                                joker.into(),
                                (content_cloned.unwrap()).into(),
                
                                //String::from_utf8(content.into().unwrap()),
                                //content,
                                br(),

                                a(&[href("#"), "My Blog <hello world />".into()]),

                                br(),
                                br()
                            ]) //closed div                                 
                        ])
                    ])
                ]); 	

                match content {                  
                    Some(_content) => Response::html(frag.data),
                    None => Response::empty_404(),
                }
            },



            (PUT) (/note/{id: i32}) => {
                // This route modifies the content of an existing note.

                // We start by reading the body of the HTTP request into a `String`.
                let body = try_or_400!(rouille::input::plain_text_body(&request));

                // And write the content with a query. This line can only panic if the
                // SQL is malformed.
                let updated = db.execute("UPDATE notes SET content = $2 WHERE id = $1",
                                         &[&id, &body]).unwrap();

                // We determine whether the note existed based on the number of rows that
                // were modified.
                if updated >= 1 {
                    Response::text("The note has been updated")
                } else {
                    Response::empty_404()
                }
            },

            (POST) (/note) => {
                // This route creates a new note whose initial content is the body.

                // We start by reading the body of the HTTP request into a `String`.
                let body = try_or_400!(rouille::input::plain_text_body(&request));

                // To do so, we first create a variable that will receive the content.
                let mut id: Option<i32> = None;
                // And then perform the query and write to `content`. This line can only panic if the
                // SQL is malformed.
                for row in &db.query("INSERT INTO notes(content) VALUES ($1) RETURNING id", &[&body]).unwrap() {
                    id = Some(row.get(0));
                }

                let mut response = Response::text(format!("{}", body));

                response.status_code = 201;

                response.headers.push(("Location".into(), format!("/note/{}", id.unwrap()).into()));

                response
            },

            (DELETE) (/note/{id: i32}) => {
                // This route deletes a note. This line can only panic if the
                // SQL is malformed.
                db.execute("DELETE FROM notes WHERE id = $1", &[&id]).unwrap();
                Response::text("")
            },

            // If none of the other blocks matches the request, return a 404 response.
            _ => Response::empty_404()
    )
}
