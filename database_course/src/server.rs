pub mod info {
    use std::time::SystemTime;
    pub struct FlightInfo {
        pub fid: String,
        pub begin_time: SystemTime,
        pub end_time: SystemTime,
        pub begin_airport: String,
        pub end_airport: String,
    }
    pub struct SeatInfo {
        pub sid: String,
        pub seat_type: i32,
        pub price: f32
    }
    pub struct OrderInfo {
        pub oid: String,
        pub tid: String
    }
    pub struct TicketInfo {
        pub tid: String,
        pub name: String,
        pub fid: String,
        pub time: SystemTime,
        pub bplace: String,
        pub eplace: String,
        pub sid: String
    }
    pub struct CurrentUserInfo {
        pub uid: i64,
        pub power: i32,
        // power: 0 means non-logged
        // power: 1 means logged
        // power: 2 means admin
    }
    impl CurrentUserInfo {
        pub fn new() -> Self {
            CurrentUserInfo {
                uid: 0,
                power: 0
            }
        }
    }
}

pub mod operators {
    use crate::connect_database::Connection;
    use super::info::*;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    pub fn user_login(connect: &mut Connection, uid: i64, pwd: &String, current_user: &mut CurrentUserInfo) -> bool {
        if connect.check_pwd(uid, pwd) && connect.check_user_usable(uid) {
            current_user.power = 1;
            current_user.uid = uid;
            true
        } else {
            false
        }
    }
    pub fn user_regist(connect: &mut Connection, uname: &String, pwd: &String, current_user: &mut CurrentUserInfo) -> Result<i64, postgres::Error> {
        if connect.check_name_usable(uname) {
            let mut hasher = DefaultHasher::new();
            uname.hash(&mut hasher);
            let mut uid = hasher.finish() as i64;
            if uid < 0 {
                uid = -uid;
            }
            connect.add_user(uid, uname, pwd)?;
            current_user.uid = uid;
            current_user.power = 1;
            return Ok(uid);
        } else {
            println!("error rename");
        }
        Ok(0)
    }
    pub fn user_change_pwd(connect: &mut Connection, old_pwd: &String, new_pwd: &String, current_user: &CurrentUserInfo) -> Result<bool, postgres::Error> {
        if current_user.power == 0 {
            return Ok(false);
        }
        if connect.check_pwd(current_user.uid, old_pwd) {
            connect.change_pwd(current_user.uid, new_pwd)?;
            return Ok(true);
        }
        Ok(false)
    }
    pub fn user_logout(current_user: &mut CurrentUserInfo) {
        current_user.uid = 0;
        current_user.power = 0;
    }
    pub fn admin_login(connect: &mut Connection, uid: i64, pwd: &String, current_user: &mut CurrentUserInfo) -> bool {
        if connect.check_admin_pwd(uid, pwd) {
            current_user.power = 2;
            current_user.uid = uid;
            true
        } else {
            false
        }
    }
    pub fn admin_set_user(connect: &mut Connection, set_user_id: i64, set_status: bool, current_user: &CurrentUserInfo) -> Result<bool, postgres::Error> {
        if current_user.power != 2 {
            return Ok(false);
        }
        connect.set_user(set_user_id, set_status)?;
        Ok(true)
    }
    pub fn admin_set_flight(connect: &mut Connection, set_flight_id: &String, set_status: bool, current_user: &CurrentUserInfo) -> Result<bool, postgres::Error> {
        if current_user.power != 2 {
            return Ok(false);
        }
        connect.set_flight(set_flight_id, set_status)?;
        Ok(true)
    }
    pub fn query_flight(connect: &mut Connection, begin_airport: &String, end_airport: &String) -> Result<Vec<FlightInfo>, postgres::Error> {
        connect.flight_query(begin_airport, end_airport)
    }
    pub fn query_seat(connect: &mut Connection, fid: &String) -> Result<Vec<SeatInfo>, postgres::Error> {
        connect.seat_query(fid)
    }
    pub fn user_make_order(connect: &mut Connection, fid: &String, sid: &String, ticket_owner: &String, current_user: &CurrentUserInfo) -> Result<String, postgres::Error> {
        let mut hasher = DefaultHasher::new();
        format!("{}_{}", fid, sid).hash(&mut hasher);
        let oid = hasher.finish().to_string();
        oid.clone().hash(&mut hasher);
        let tid = hasher.finish().to_string();       
        connect.add_order(&oid, current_user.uid, &tid, ticket_owner, fid, sid)?;
        Ok(oid)
    } 
    pub fn user_cancel_order(connect: &mut Connection, oid: &String) -> Result<(), postgres::Error> {
        connect.cancel_order(oid)
    }
    pub fn user_show_order(connect: &mut Connection, current_user: &CurrentUserInfo) -> Result<Vec<OrderInfo>, postgres::Error> {
        connect.order_query(current_user.uid)
    }
    pub fn user_show_ticket(connect: &mut Connection, tid: &String) -> Result<Vec<TicketInfo>, postgres::Error> {
        connect.ticket_query(tid)
    }
}
