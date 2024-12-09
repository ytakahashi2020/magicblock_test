### 1 Deploy the sample counter program

1 git clone 
git clone https://github.com/ytakahashi2020/magicblock_test.git

2 ローカルでバリデータを立ち上げる
solana-test-validator

3 Build
anchor build

4 Deploy
anchor deploy

### 2 Dep

1 set dependencies

ephemeral-rollups-sdk = { version = "0.0.6", features = ["anchor"] }

2 set #[delegate] on the program
#[delegate]

3 set DelegateInput Struct

4 set the delegate function
