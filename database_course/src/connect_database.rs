use postgres::{Client, Error, NoTls};
use postgres::{IsolationLevel};
use super::genator::*;
use super::server::info::*;

pub struct Connection {
    pub client: Client,
}
impl Connection {
    pub fn new() -> Result<Self, Error> {
        let connect_config: String = format!(
            "postgresql://postgres:hat@127.0.0.1:5432/Airplane_ticket",
        );
        let client = Client::connect(&connect_config, NoTls)?;
        Ok(Connection { client })
    }
    pub fn disconnect(self) -> Result<(), Error> {
        if !self.client.is_closed() {
            self.client.close()?;
        }
        Ok(())
    }
    // operator of everyone 
    pub fn flight_query(&mut self, begin_airport: &String, end_airport: &String) -> Result<Vec<FlightInfo>, Error> {
        let mut res = vec![];
        let para1 = format!("{}", begin_airport);
        let para2 = format!("{}", end_airport);
        let para1 = &para1[0..2];
        let para2 = &para2[0..2];
        println!("{}:{}, {}:{}", para1, para1.len(), para2, para2.len());
        for row in self.client.query("SELECT * FROM flight_table WHERE bplace = $1 AND dplace = $2", &[&para1, &para2])? {
            if row.get(5) {
                res.push(FlightInfo{
                    fid: row.get(0),
                    begin_time: row.get(1),
                    end_time: row.get(2),
                    begin_airport: row.get(3),
                    end_airport: row.get(4)
                })
            }
        }
        Ok(res)
    }

    // operator of user
    pub fn add_user(&mut self, uid: i64, uname: &String, pwd: &String) -> Result<(), Error> {
        // sign up a user
        self.client
            .batch_execute(&add_user_sql(uid, uname, pwd))?;        
        Ok(())
    }
    pub fn check_name_usable(&mut self, uname: &String) -> bool {
        if let Ok(_row) = self.client.query_one(&check_name_usable_sql(uname)[..], &[]) {
            return false;
        } else {
            return true;
        }
    }
    pub fn check_user_usable(&mut self, uid: i64) -> bool {
        // check user's usability
        if let Ok(row) = self.client.query_one(&check_user_sql(uid)[..], &[]) {
            if row.get(0) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    pub fn seat_query(&mut self, fid: &String) -> Result<Vec<SeatInfo>, Error> {
        let mut res = vec![];
        let para1 = &format!("{}", fid)[0..2];
        for row in self.client.query("SELECT sid, type, price FROM seat_table WHERE fid = $1 AND usable = true", &[&para1])? {
            res.push(SeatInfo{
                sid: row.get(0),
                seat_type: row.get(1),
                price: row.get(2)
            })
        }
        Ok(res)
    }
    pub fn order_query(&mut self, uid: i64) -> Result<Vec<OrderInfo>, Error> {
        let mut res = vec![];
        for row in self.client.query(&order_query_sql(uid)[..], &[])? {
            res.push(OrderInfo{
                oid: row.get(0),
                tid: row.get(1)
            })
        }
        Ok(res)
    }
    pub fn ticket_query(&mut self, tid: &String) -> Result<Vec<TicketInfo>, Error> {
        let mut res = vec![];
        for row in self.client.query(&ticket_query_sql(tid)[..], &[])? {
            let fid: String = row.get(2);
            let info = self.client.query_one(&flight_query_by_fid_sql(&fid)[..], &[])?;
            res.push(TicketInfo{
                tid: row.get(0),
                name: row.get(1),
                fid,
                time: info.get(0),
                bplace: info.get(1),
                eplace: info.get(2),
                sid: row.get(3)
            })
        }
        Ok(res)
    }
    pub fn check_pwd(&mut self, uid: i64, pwd: &String) -> bool {
        // check user's password
        if let Ok(row) = self.client.query_one(&pwd_query_sql(uid)[..], &[]) {
            let true_pwd: String = row.get(0);
            if &true_pwd == pwd.trim() {
                return true;
            } else {
                println!("tp: {}, p: {}", true_pwd.len(), pwd.len());
                return false;
            }
        } else {
            println!("panic");
            return false;
        }
    }
    pub fn change_pwd(&mut self, uid: i64, new_pwd: &String) -> Result<(), Error> {
        // user changes password
        self.client.batch_execute(&change_pwd_sql(uid, new_pwd))
    }
    pub fn add_order(&mut self, oid: &String, uid: i64, tid: &String, ticket_owner: &String, fid: &String, sid: &String) -> Result<(), Error> {
        let mut transaction = self.client.build_transaction().isolation_level(IsolationLevel::RepeatableRead).start()?;
        transaction.batch_execute(&add_order_sql(oid, uid, tid))?;
        transaction.batch_execute(&add_ticket_sql(tid, ticket_owner, fid, sid))?;
        transaction.commit()
    }
    pub fn cancel_order(&mut self, oid: &String) -> Result<(), Error> {
        let mut transaction = self.client.build_transaction().isolation_level(IsolationLevel::RepeatableRead).start()?;
        transaction.batch_execute(&cancel_order_sql(oid))?;
        transaction.commit()
    }
    // operator of admin
    pub fn set_user(&mut self, uid: i64, new_status: bool) -> Result<(), Error> {
        // admin bans or unbans user
        self.client.batch_execute(&set_user_sql(uid, new_status))
    }
    pub fn set_flight(&mut self, fid: &String, new_status: bool) -> Result<(), Error> {
        self.client.batch_execute(&set_flight_sql(fid, new_status))
    }
    pub fn check_admin_pwd(&mut self, uid: i64, pwd: &String) -> bool {
        if let Ok(row) = self.client.query_one(&admin_pwd_check_sql(uid)[..], &[]) {
            let true_pwd: String = row.get(0);
            if &true_pwd == pwd {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}
