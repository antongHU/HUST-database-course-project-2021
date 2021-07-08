mod connect_database;
mod genator;
mod server;
use server::info::*;
use server::operators::*;
use connect_database::{Connection};
use chrono::offset::Utc;
use chrono::DateTime;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
fn handle_stream(mut stream: TcpStream, connection: &mut Connection, user: &mut CurrentUserInfo) {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();
    let requset = String::from_utf8_lossy(&buffer[..]);
    println!("{}", requset);
    let mut itor = requset.split_whitespace();
    let cnt = itor.clone().count() - 2;
    if itor.next().unwrap() == String::from("GET") {
        let mut request_path = String::from(itor.next().unwrap());
        if request_path == String::from("/") {
            request_path = "./html/index.html".to_string();
        } else if request_path == String::from("/favicon.ico") {
            return;
        } else if request_path == String::from("/sign_out.html") {
            user_logout(user);
            request_path = format!("./html{}", request_path);
        } else {
            request_path = format!("./html{}", request_path);
        }
        let contest = fs::read_to_string(request_path).unwrap();
        let contest = format!("HTTP/1.1 200 OK\r\n\r\n{}", contest);
        stream.write(contest.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let request_path = String::from(itor.next().unwrap());
        let post = String::from(itor.nth(cnt - 1).unwrap());
        let info_vec: Vec<String> = post.split('&').map(|x| String::from(x)).collect();
        if request_path == String::from("/user_sign_in.html") {
            for i in &info_vec {
                println!("{}", i);
            }
            let uid: i64 = String::from(&info_vec[0][4..]).parse().unwrap();
            let pwd: String = String::from(&info_vec[1][4..7]).trim().to_string();
            println!("uid: {}\npwd: {}", uid, pwd);
            if !user_login(connection, uid, &pwd, user) {
                let contest = fs::read_to_string("./html/login_failed.html").unwrap();
                let contest = format!("HTTP/1.1 200 OK\r\n\r\n{}", contest);
                stream.write(contest.as_bytes()).unwrap();
                stream.flush().unwrap();
            } else {
                let contest = fs::read_to_string("./html/user_mainpage.html").unwrap();
                let contest = format!("HTTP/1.1 200 OK\r\n\r\n{}", contest);
                stream.write(contest.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        } else if request_path == String::from("/user_sign_up.html") {
            let uname = String::from(&info_vec[0][6..9]);
            let pwd: String = String::from(&info_vec[1][4..7]);
            println!("{}, {}", uname, pwd);
            if let Ok(uid) = user_regist(connection, &uname, &pwd, user) {
                let contest = format!("HTTP/1.1 200 OK\r\n\r\n<!DOCTYPE html>
                <html>
                <head>
                <meta charset=\"utf-8\">
                <title>Success!</title>
                </head>
                <body>
                    <h1>
                        Welcome
                    </h1>
                    <p>
                        Your uid is {}
                    </p>
                    <p>
                        <a href=\"user_mainpage.html\">Go to mainpage</a>
                    </p>    
                </body>
                </html>
                ", uid);
                stream.write(contest.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        } else if request_path == String::from("/admin_sign_in.html") {
            let uid: i64 = String::from(&info_vec[0][4..]).parse().unwrap();
            let pwd: String = String::from(&info_vec[1][4..9]);
            if !admin_login(connection, uid, &pwd, user) {
                let contest = fs::read_to_string("./html/login_failed.html").unwrap();
                let contest = format!("HTTP/1.1 200 OK\r\n\r\n{}", contest);
                stream.write(contest.as_bytes()).unwrap();
                stream.flush().unwrap();
            } else {
                let contest = fs::read_to_string("./html/admin_mainpage.html").unwrap();
                let contest = format!("HTTP/1.1 200 OK\r\n\r\n{}", contest);
                stream.write(contest.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        } else if request_path == String::from("/airplane_query_user.html") {
            let origin = String::from(&info_vec[0][4..]);
            let destination = String::from(&info_vec[1][4..]);
            let res = query_flight(connection, &origin, &destination).unwrap();
            let mut contest = format!("HTTP/1.1 200 OK\r\n\r\n<!DOCTYPE html>
                <html>
                <head>
                <meta charset=\"utf-8\">
                <title>Airplane Query</title>
                </head>
                <body>
                <h1>
                    Query Result:
                </h1>
                <table border=\"1\">
                <tr>
                    <th>Flight ID</th>
                    <th>Takeoff Time</th>
                    <th>Landing Time</th>
                    <th>Origin Airport</th>
                    <th>Dest Atrport</th>
                </tr>\n
            ");
            for info in res {
                let begin_time: DateTime<Utc> = info.begin_time.into();
                let end_time: DateTime<Utc> = info.end_time.into();
                contest = contest + &format!("
                    <tr>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                    </tr>\n
                ", info.fid, begin_time.format("%d/%m/%Y %T"), end_time.format("%d/%m/%Y %T"), info.begin_airport, info.end_airport)[..];
            }
            contest = contest + &format!("
                </table>
                <p>
                    <a href=\"seat_query.html\">Query Seat</a>
                </p>
                </body>
                </html>
            ")[..];
            stream.write(contest.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else if request_path == String::from("/seat_query.html") {
            let fid = String::from(&info_vec[0][4..]);
            let res = query_seat(connection, &fid).unwrap();
            let mut contest = format!("HTTP/1.1 200 OK\r\n\r\n<!DOCTYPE html>
                <html>
                <head>
                <meta charset=\"utf-8\">
                <title>Seat Query</title>
                </head>
                <body>
                <h1>
                    Query Result:
                </h1>
                <table border=\"1\">
                <tr>
                    <th>Seat ID</th>
                    <th>Seat type</th>
                    <th>Price</th>
                </tr>\n
            ");
            for info in res {
                contest = contest + &format!("
                    <tr>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                    </tr>\n
                ", info.sid, info.seat_type, info.price)[..];
            }
            contest = contest + &format!("
                </table>
                <p>
                    <a href=\"order_ticket.html\">Make Order</a>
                </p>
                </body>
                </html>
            ")[..];
            stream.write(contest.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else if request_path == String::from("/order_ticket.html") {
            let fid = String::from(&info_vec[0][4..6]);
            let sid = String::from(&info_vec[1][4..7]);
            let name = String::from(&info_vec[2][5..8]);
            let oid = user_make_order(connection, &fid, &sid, &name, user).unwrap();
            let contest = format!("HTTP/1.1 200 OK\r\n\r\n<!DOCTYPE html>
                <html>
                <head>
                <meta charset=\"utf-8\">
                <title>Success</title>
                </head>
                <body>
                <h1>
                    You have made an order
                </h1>
                <p>
                    Your order ID is {}
                </p>
                <p>
                <a href=\"user_mainpage.html\">Back to mainpage</a>
                </p>
                </body>
                </html>
            ", oid);
            stream.write(contest.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else if request_path == String::from("/airplane_query_none.html") {
            let origin = String::from(&info_vec[0][4..]).trim().to_string();
            let destination = String::from(&info_vec[1][4..]).trim().to_string();
            let res = query_flight(connection, &origin, &destination).unwrap();
            let mut contest = format!("HTTP/1.1 200 OK\r\n\r\n<!DOCTYPE html>
                <html>
                <head>
                <meta charset=\"utf-8\">
                <title>Airplane Query</title>
                </head>
                <body>
                <h1>
                    Query Result:
                </h1>
                <table border=\"1\">
                <tr>
                    <th>Flight ID</th>
                    <th>Takeoff Time</th>
                    <th>Landing Time</th>
                    <th>Origin Airport</th>
                    <th>Dest Atrport</th>
                </tr>\n
            ");
            for info in res {
                let begin_time: DateTime<Utc> = info.begin_time.into();
                let end_time: DateTime<Utc> = info.end_time.into();
                contest = contest + &format!("
                    <tr>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                    </tr>\n
                ", info.fid, begin_time.format("%d/%m/%Y %T"), end_time.format("%d/%m/%Y %T"), info.begin_airport, info.end_airport)[..];
            }
            contest = contest + &format!("
                </table>
                <p>
                    <a href=\"index.html\">Back to Index</a>
                </p>
                </body>
                </html>
            ")[..];
            stream.write(contest.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else if request_path == String::from("/ticket_query.html") {
            let oid = String::from(&info_vec[0][4..]);
            let mut para = String::new();
            for ch in oid.chars() {
                if ch == '0' || ch == '1' || ch == '2' || ch == '3' || ch == '4' || ch == '5' || ch == '6' || ch == '7' || ch == '8' || ch == '9' {
                    para.push(ch);
                } else {
                    break;
                }
            }
            let res = user_show_ticket(connection, &para).unwrap();
            let mut contest = format!("HTTP/1.1 200 OK\r\n\r\n<!DOCTYPE html>
                <html>
                <head>
                <meta charset=\"utf-8\">
                <title>Ticket Query</title>
                </head>
                <body>
                <h1>
                    Query Result:
                </h1>
                <table border=\"1\">
                <tr>
                    <th>Ticket ID</th>
                    <th>Passenger Name</th>
                    <th>Flight ID</th>
                    <th>TakeOff Time</th>
                    <th>Origin Airport</th>
                    <th>Dest Airport</th>
                    <th>Seat ID</th>
                </tr>\n
            ");
            for info in res {
                let time: DateTime<Utc> = info.time.into();
                contest = contest + &format!("
                    <tr>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{}</td>
                    </tr>\n
                ", info.tid, info.name, info.fid, time.format("%d/%m/%Y %T"), info.bplace, info.eplace, info.sid)[..];
            }
            contest = contest + &format!("
                </table>
                <p>
                    <a href=\"user_mainpage.html\">Back to Mainpage</a>
                </p>
                </body>
                </html>
            ")[..];
            stream.write(contest.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else if request_path == String::from("/airplane_usable_set.html") {
            let fid = String::from(&info_vec[0][4..]);
            let usable = if &info_vec[1][4..8] == "true" {
                true
            } else {
                false
            };
            admin_set_flight(connection, &fid, usable, user).unwrap();
            let contest = format!("HTTP/1.1 200 OK\r\n\r\n<!DOCTYPE html>
                <html>
                <head>
                <meta charset=\"utf-8\">
                <title>Success</title>
                </head>
                <body>
                <h1>
                    Operation Finished
                </h1>
                <p>
                    <a href=\"admin_mainpage.html\">Back to Mainpage</a>
                </p>
                </body>
                </html>
            ");
            stream.write(contest.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else if request_path == String::from("/user_usable_set.html") {
            let uid: i64 = String::from(&info_vec[0][4..]).parse().unwrap();
            let usable = if &info_vec[1][4..8] == "true" {
                true
            } else {
                false
            };
            admin_set_user(connection, uid, usable, user).unwrap();
            let contest = format!("HTTP/1.1 200 OK\r\n\r\n<!DOCTYPE html>
                <html>
                <head>
                <meta charset=\"utf-8\">
                <title>Success</title>
                </head>
                <body>
                <h1>
                    Operation Finished
                </h1>
                <p>
                    <a href=\"admin_mainpage.html\">Back to Mainpage</a>
                </p>
                </body>
                </html>
            ");
            stream.write(contest.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else if request_path == String::from("/order_query.html") {
            let res = user_show_order(connection, user).unwrap();
            let mut contest = format!("HTTP/1.1 200 OK\r\n\r\n<!DOCTYPE html>
                <html>
                <head>
                <meta charset=\"utf-8\">
                <title>Ticket Query</title>
                </head>
                <body>
                <h1>
                    Query Result:
                </h1>
                <table border=\"1\">
                <tr>
                    <th>Order ID</th>
                    <th>Ticket ID</th>
                </tr>\n
            ");
            for info in res {
                contest = contest + &format!("
                    <tr>
                        <td>{}</td>
                        <td>{}</td>
                    </tr>\n
                ", info.oid, info.tid)[..];
            }
            contest = contest + &format!("
                </table>
                <p>
                    <a href=\"order_cancel.html\">Cancel Order</a>
                </p>
                <p>
                    <a href=\"user_mainpage.html\">Back to Mainpage</a>
                </p>
                </body>
                </html>
            ")[..];
            stream.write(contest.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else if request_path == String::from("/user_pwd_change.html") {
            // let uid: i64 = String::from(&info_vec[0][3..]).parse().unwrap();
            let old_pwd = String::from(&info_vec[1][5..8]);
            let new_pwd = String::from(&info_vec[2][5..8]);
            user_change_pwd(connection, &old_pwd, &new_pwd, user).unwrap();
            let contest = format!("HTTP/1.1 200 OK\r\n\r\n<!DOCTYPE html>
                <html>
                <head>
                <meta charset=\"utf-8\">
                <title>Success</title>
                </head>
                <body>
                <h1>
                    Operation Finished
                </h1>
                <p>
                    <a href=\"user_mainpage.html\">Back to Mainpage</a>
                </p>
                </body>
                </html>
            ");
            stream.write(contest.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else if request_path == String::from("/order_cancel.html") {
            // let uid: i64 = String::from(&info_vec[0][3..]).parse().unwrap();
            let oid = String::from(&info_vec[0][4..]);
            let mut para = String::new();
            for ch in oid.chars() {
                if ch == '0' || ch == '1' || ch == '2' || ch == '3' || ch == '4' || ch == '5' || ch == '6' || ch == '7' || ch == '8' || ch == '9' {
                    para.push(ch);
                } else {
                    break;
                }
            }
            user_cancel_order(connection, &para).unwrap();
            let contest = format!("HTTP/1.1 200 OK\r\n\r\n<!DOCTYPE html>
                <html>
                <head>
                <meta charset=\"utf-8\">
                <title>Success</title>
                </head>
                <body>
                <h1>
                    Operator Finished
                </h1>
                <p>
                    <a href=\"user_mainpage.html\">Back to Mainpage</a>
                </p>
                </body>
                </html>
            ");
            stream.write(contest.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
fn main() -> Result<(), postgres::Error> {
    let listener = TcpListener::bind("127.0.0.1:14514").unwrap();
    let mut connection = Connection::new()?;
    let mut current_user = CurrentUserInfo::new();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream, &mut connection, &mut current_user);
    }
    connection.disconnect()?;
    Ok(())
}
