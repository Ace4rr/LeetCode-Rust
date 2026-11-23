fn main() {
    println!("Hello, world!");
    #[derive(Debug, Clone)]
    pub struct Account{
        pub id: u64,
        pub owner: String,
        pub balance: i64,
    }
    impl Account{
        pub fn new(id:u64, owner:String)->Self{
            Self{
                id,
                owner,
                balance:0,
            }
        }

        pub fn deposit(&mut self,amount:i64) {
            self.balance+=amount;
        }

        pub fn get_balance(&self)->i64{
            self.balance
        }
    }

}
