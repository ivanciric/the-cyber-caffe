use percentage::Percentage;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{log, near_bindgen, env, Promise, AccountId, ext_contract};
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::{Token, TokenId};

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;
pub const SHOP_OWNER: &str = "hamato.testnet";
pub const NFT_CONTRACT: &str = "cynt-nft-1.testnet";
pub const OWNER_FEE_PERCENT: i32 = 50;

// Let's create an interface for our external contract
#[ext_contract(ext_coffeenft)]
trait Coffeenft {
    fn new_default_meta(
        owner_id: AccountId
    ) -> Self;

    fn nft_mint(&mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> Token ;
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    available_coffees: UnorderedMap<String, f32>,
    // tried using AccountId instead of String, but that caused serialization errors as AccountId does not implemet the Display trait
    patrons: UnorderedMap<AccountId, f32>,
    nft_images: UnorderedMap<String, String>
}

// We just set initial values for our struct, empty UnorderedMaps
impl Default for Contract {
    fn default() -> Self {
        Self{
            available_coffees: UnorderedMap::new(b"m"),
            patrons: UnorderedMap::new(b"d"),
            nft_images: UnorderedMap::new(b"c")
        }
    }
}

#[near_bindgen]
impl Contract {
    /**
     * When we deploy the contract, owner needs to call this function
     * in order to populate the available_coffees map.
     * At start, we assert that only owner can execute the function.
     * 
     * At the same time, we populate images for coffee nfts (stored on ipfs via nft.storage)
     */
    pub fn populate_menu(&mut self) -> Vec<(String, f32)> {
        assert_eq!(env::predecessor_account_id(), SHOP_OWNER.parse().unwrap(), "Only owner can populate menu");
        
        self.available_coffees.clear();
        self.available_coffees.insert(&"Espresso".to_string(), &4.2);
        self.available_coffees.insert(&"Americano".to_string(), &5.4);
        self.available_coffees.insert(&"Latte".to_string(), &7.6);
        self.available_coffees.insert(&"Cappuccino".to_string(), &7.2);

        self.nft_images.clear();
        self.nft_images.insert(&"Espresso".to_string(), &"https://bafybeifx3fp3koly36bazuo3rq5fp6jd24lzchoiyi3uo45fha6yymumei.ipfs.nftstorage.link".to_string());
        self.nft_images.insert(&"Americano".to_string(), &"https://bafybeicrymgstxlbaqt25bnfjhe5c3brx3zqbfcouj5rhki37awvg3d46i.ipfs.nftstorage.link".to_string());
        self.nft_images.insert(&"Latte".to_string(), &"https://bafybeigylthc3osc6bxmv4sv7lz4l263lp4rdxazidl3swktdnbniyes7y.ipfs.nftstorage.link".to_string());
        self.nft_images.insert(&"Cappuccino".to_string(), &"https://bafybeifni7gxmatj4vkj2grftar7jvjuqb5e7lp2nwtjlo3fr6i5couoga.ipfs.nftstorage.link".to_string());

        log!("Menu and nft images populated");
        self.available_coffees.to_vec()
    }

    // Init NFT contract to set itself as owner in order to be able to mint
    pub fn initialize_nft_contract(&self) {
        let nft_contract: AccountId = NFT_CONTRACT.parse().unwrap();
        ext_coffeenft::ext(nft_contract).new_default_meta(env::current_account_id());
        log!("NFT contract initialized");
    }

    // Mint tne NFT, calling the nft_mint function on the nft contract
    // This method can be run from the app dashboard if you are the shop owner
    // We just use it to test if everything works when contract is deployed
    pub fn mint_test_coffee_nft(&self, coffee: String, deposit: String, id: String) {
        assert_eq!(env::predecessor_account_id(), SHOP_OWNER.parse().unwrap(), "Only owner can test nft minting");

        let nft_contract: AccountId = NFT_CONTRACT.parse().unwrap();
        let nft_image = &self.nft_images.get(&coffee).unwrap_or("https://bafybeifx3fp3koly36bazuo3rq5fp6jd24lzchoiyi3uo45fha6yymumei.ipfs.nftstorage.link".to_string());
        let token_metdata = TokenMetadata { 
            title: Some(coffee), 
            description: Some("The Cyber Caffe NFT Colllection".to_string()), 
            media: Some(nft_image.to_string()), 
            copies: Some(1),
            media_hash: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None
        };
        let d = deposit.parse::<u128>().unwrap();
        ext_coffeenft::ext(nft_contract).with_attached_deposit(d).nft_mint(
            id,
            env::predecessor_account_id(),
            token_metdata
        );

        log!("NFT minted");
    }

    // Only this contract can call this function
    #[private]
    pub fn mint_coffee_nft(
        &self, 
        coffee: String, 
        strength: String,
        id: String,
        deposit: u128
    ) {

        let nft_contract: AccountId = NFT_CONTRACT.parse().unwrap();
        
        let nft_image = &self.nft_images.get(&coffee).unwrap_or("https://bafybeifx3fp3koly36bazuo3rq5fp6jd24lzchoiyi3uo45fha6yymumei.ipfs.nftstorage.link".to_string());
        
        let token_metdata = TokenMetadata { 
            title: Some(coffee), 
            description: Some("The Cyber Caffe NFT Colllection".to_string()), 
            media: Some(nft_image.to_string()), 
            copies: Some(1),
            media_hash: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: Some(strength),
            reference: None,
            reference_hash: None
        };
      
        // hardcoded storage cost until I figure out why parameter is not working
        ext_coffeenft::ext(nft_contract).with_attached_deposit(deposit).nft_mint(
            id,
            env::predecessor_account_id(),
            token_metdata
        );
    }

    // We just return our struct property as vector
    pub fn get_available_coffees(&self) -> Vec<(String, f32)> {
        self.available_coffees.to_vec()
    }
    
    /**
     * Adding new element to UnorderedMap.
     * First we check that the owner is calling this function and assert some basic stuff as well
     */
    pub fn add_new_coffee(&mut self, name: String, price: String, img_url: String) {
        assert_eq!(env::predecessor_account_id(), SHOP_OWNER.parse().unwrap(), "Only owner can add new items");
        assert_eq!(name.is_empty(), false, "Name cannot be empty");
        assert_eq!(price.is_empty(), false, "Price cannot be empty");
        assert_eq!(img_url.is_empty(), false, "Image url cannot be empty");
   
        let exists = self.available_coffees.get(&name).unwrap_or(0.0);

        if exists == 0.0 {
            self.available_coffees.insert(&name, &price.parse::<f32>().unwrap());
            self.nft_images.insert(&name, &img_url);
            log!("{} coffee saved", &name);
        } else {
            log!("The {} coffe is already on the menu. Add a different one or call the update_price function to change the price", &name);
        }
    }

    // Another 'private' function, which updates an element of the UnorderedMap
    pub fn update_price(&mut self, name: String, price: String) {
        assert_eq!(env::predecessor_account_id(), SHOP_OWNER.parse().unwrap(), "Only owner can update prices");
        
        self.available_coffees.insert(&name, &price.parse::<f32>().unwrap());
        log!("Price for {} updated to {}", &name, &price);
    }

    // Public function to check how much has account_id already paid through our app
    pub fn get_patron_spent_amount(&self, account_id: String) -> f32 {
        let existing_customer_already_spent = self.patrons.get(&account_id.parse().unwrap()).unwrap_or(0.0);
        existing_customer_already_spent
    }

    /**
     * Main function of the contract, actual payment is happening here.
     * We get the account of the sender, check if we have them in our map from before,
     * convert amount sent to NEAR (u128 to f32) for convenience.
     * After that we add the new amount to his previous spending, and in the end we return 
     * his total spenditure from the app.
     * 
     * Before returning total spent for the account, we execute the transfer to the app SHOP_OWNER.
     * We don't use the actual contract account but define a separate in a constant at the begining of file.
     */
    #[payable]
    pub fn buy_coffee(&mut self, name: String, id: String) -> f32 {
        let buyer = &env::predecessor_account_id();
        let existing_customer_already_spent = self.patrons.get(&buyer).unwrap_or(0.0);
        let spent_now = (env::attached_deposit() / STORAGE_COST) as f32 / 1000.00;

        let new_amount = existing_customer_already_spent + spent_now;
        self.patrons.insert(&buyer, &new_amount);

        // pay the shop owner a preset percentage
        let owner_fee_percent = Percentage::from(OWNER_FEE_PERCENT);
        let owner_fee = owner_fee_percent.apply_to(env::attached_deposit());
        Promise::new(SHOP_OWNER.parse().unwrap()).transfer(owner_fee);

        // route remaining deposit
        let remaining_deposit = env::attached_deposit() - owner_fee;
        let minting_deposit_percent = Percentage::from(50);
        let minting_deposit = minting_deposit_percent.apply_to(remaining_deposit);

        // serve coffee to patron, with adequate strength based on his total spent amount
        let strength = format!("Strength: {}", new_amount);
        self.mint_coffee_nft(name, strength, id, minting_deposit);

        new_amount
    }  
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env};

    // A function to help setup env context
    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    #[should_panic]
    fn only_owner_can_populate_menu() {
         // Init the contract
        let mut contract = Contract {
            available_coffees: UnorderedMap::new(b"m"),
            patrons: UnorderedMap::new(b"d"),
            nft_images: UnorderedMap::new(b"c") 
        };

        // This should panic since a random test account is calling the function
        contract.populate_menu();
    }

    // This covers get_available_coffees() as well
    #[test]
    fn menu_populated_correctly() {
        // Since only a specified account should execute this function, we set the environment to use the propper owner
        let context = get_context(SHOP_OWNER.parse().unwrap());
        testing_env!(context.build());

        // Init the contract
        let mut contract = Contract {
            available_coffees: UnorderedMap::new(b"m"),
            patrons: UnorderedMap::new(b"d"),
            nft_images: UnorderedMap::new(b"c") 
        };

        // We check if the menu was populated with preset data
        let available_coffees = contract.populate_menu();
        if let Some((key, _value)) = available_coffees.first() {
            assert_eq!(key, "Espresso");
        }
    }
    #[test]
    fn only_owner_can_add_new_coffee() {

        // Since only a specified account should execute this function, we set the environment to use the propper owner
        let context = get_context(SHOP_OWNER.parse().unwrap());
        testing_env!(context.build());

        // Init the contract
       let mut contract = Contract {
            available_coffees: UnorderedMap::new(b"m"),
            patrons: UnorderedMap::new(b"d"),
            nft_images: UnorderedMap::new(b"c") 
        };

       contract.add_new_coffee("Test".to_string(), "2.2".to_string(), "test.url".to_string());
   }
   #[test]
   #[should_panic]
   fn non_owner_can_not_add_new_coffee() {

    // Init the contract
    let mut contract = Contract {
            available_coffees: UnorderedMap::new(b"m"),
            patrons: UnorderedMap::new(b"d"),
            nft_images: UnorderedMap::new(b"c") 
        };

    contract.add_new_coffee("Test".to_string(), "2.2".to_string(), "test.url".to_string());
    }
   
    // Owner can update price of existing coffee
    #[test]
    fn owner_can_update_price_of_existing_coffee() {

        // Since only a specified account should execute this function, we set the environment to use the propper owner
        let context = get_context(SHOP_OWNER.parse().unwrap());
        testing_env!(context.build());

        // Init the contract
        let mut contract = Contract {
            available_coffees: UnorderedMap::new(b"m"),
            patrons: UnorderedMap::new(b"d"),
            nft_images: UnorderedMap::new(b"c") 
        };

        // Add new coffee and update its price afterwards
        contract.add_new_coffee("Test".to_string(), "2.2".to_string(), "test.url".to_string());
        contract.update_price("Test".to_string(), "3.3".to_string());

        let coffee = contract.available_coffees.get(&"Test".to_string()).unwrap();
        assert_eq!(coffee, 3.3);
    }

        // Non-owner account cannot update price of existing coffee
    #[test]
    #[should_panic]
    fn non_owner_cannot_update_price_of_existing_coffee() {
        // Init the contract
        let mut contract = Contract {
            available_coffees: UnorderedMap::new(b"m"),
            patrons: UnorderedMap::new(b"d"),
            nft_images: UnorderedMap::new(b"c") 
        };

        contract.update_price("Test".to_string(), "3.3".to_string());
    }

}
