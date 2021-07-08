pub fn check_user_sql(uid: i64) -> String {
    let res_sql = format!(
        "
    SELECT usable FROM custom_table WHERE uid = {}
    ",
        uid
    );
    res_sql
}
pub fn pwd_query_sql(uid: i64) -> String {
    let res_sql = format!(
        "
        SELECT pwd FROM custom_table WHERE uid = {}
    ",
        uid
    );
    res_sql
}
pub fn add_user_sql(uid: i64, uname: &String, pwd: &String) -> String {
    let res_sql = format!(
        "
        INSERT INTO custom_table VALUES ({}, '{}', '{}', true)
    ",
        uid, uname, pwd
    );
    res_sql
}
pub fn check_name_usable_sql(uname: &String) -> String {
    let res_sql = format!(
        "
        SELECT * FROM custom_table WHERE name = '{}'
    ", uname);
    res_sql
}
pub fn change_pwd_sql(uid: i64, new_pwd: &String) -> String {
    let res_sql = format!(
        "
        UPDATE custom_table
        SET pwd = '{}'
        WHERE uid = {}
    ",
        new_pwd, uid
    );
    res_sql
}
pub fn set_user_sql(uid: i64, new_status: bool) -> String {
    let res_sql = format!(
        "
        UPDATE custom_table  
        SET usable = {}
        WHERE uid = {}    
    ",
        new_status, uid
    );
    res_sql
}
// operator on flight table
// pub fn flight_query_sql(begin_airport: &String, end_airport: &String) -> String {
//     let res_sql = format!(
//         "
//         SELECT * FROM flight_table 
//         WHERE bplace = '{}' AND dplace = '{}'
//     ",
//         begin_airport, end_airport
//     );
//     res_sql
// }
pub fn flight_query_by_fid_sql(fid: &String) -> String {
    let res_sql = format!(
        "
        SELECT btime, bplace, dplace FROM flight_table
        WHERE FID = '{}'
    ", fid);
    res_sql
}
pub fn set_flight_sql(fid: &String, new_status: bool) -> String {
    let res_sql = format!(
        "
        UPDATE flight_table
        SET usable = {}
        WHERE fid = '{}'
    ",
        new_status, fid
    );
    res_sql
}
// operator on seat table
// pub fn seat_query_sql(fid: &String) -> String {
//     let res_sql = format!(
//         "
//         SELECT sid, type, price FROM seat_table WHERE fid = '{}' AND usable = true
//     ",
//         fid
//     );
//     res_sql
// }
//operator on ticket table
pub fn add_ticket_sql(
    tid: &String,
    name: &String,
    fid: &String,
    sid: &String,
) -> String {
    let res_sql = format!(
        "
        INSERT INTO ticket_table VALUES ('{}', '{}', '{}', '{}');
        UPDATE seat_table
        SET usable = false
        WHERE fid = '{}' AND sid = '{}';
    ",
        tid, name, fid, sid, fid, sid
    );
    res_sql
}
pub fn ticket_query_sql(tid: &String) -> String {
    let res_sql = format!(
        "
        SELECT * FROM ticket_table WHERE tid = '{}'
    ", tid);
    res_sql
}

//operator on order table
pub fn add_order_sql(oid: &String, uid: i64, tid: &String) -> String {
    let res_sql = format!(
        "
        INSERT INTO order_table VALUES ('{}', {}, '{}')
    ",
        oid, uid, tid
    );
    res_sql
}
pub fn order_query_sql(uid: i64) -> String {
    let res_sql = format!(
        "
        SELECT oid, tid FROM order_table WHERE uid = {}
    ",
        uid
    );
    res_sql
}
pub fn cancel_order_sql(oid: &String) -> String {
    let res_sql = format!(
        "
        DELETE FROM order_table WHERE oid = '{}'
    ",
        oid
    );
    res_sql
}
pub fn admin_pwd_check_sql(uid: i64) -> String {
    let res_sql = format!(
        "
        SELECT pwd FROM admin_table WHERE uid = {}
    ", uid);
    res_sql
}
