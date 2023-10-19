import 'regenerator-runtime/runtime';
import { Wallet } from './near-wallet';
import { formatNearAmount, parseNearAmount } from 'near-api-js/lib/utils/format';

const CONTRACT_ADDRESS = process.env.CONTRACT_NAME;

const wallet = new Wallet({ createAccessKeyFor: CONTRACT_ADDRESS })
const owner = "hamato.testnet";

window.onload = async () => {
  let isSignedIn = await wallet.startUp();

  if (isSignedIn) {
    signedInFlow();
  } else {
    signedOutFlow();
  }
  
  if(wallet.accountId == owner){
    document.querySelector('.buy-coffee').style.display = 'none';
  }
  
  if(wallet.accountId == owner && document.querySelector('#coffee_menu').options.length){
    document.querySelector('#manage_coffee').style.display = "block";
    document.querySelector('#populate_menu_button').style.display = "block";
  } else {
    document.querySelector('#manage_coffee').style.display = "none";
    document.querySelector('#populate_menu_button').style.display = "none";
  }
};

// Button clicks
document.querySelector('form#buy_coffee').onsubmit = buyCoffee;

document.querySelector('form#add_new_coffee').onsubmit = addCoffee;

document.querySelector('form#update_coffee').onsubmit = updatePrice;

document.querySelector('#populate_menu_button').onclick = populateMenu;
document.querySelector('#initialize-nft-contract').onclick = initializeNft;
document.querySelector('#mint-nft').onclick = mintNft;
document.querySelector('#sign-in-button').onclick = () => { wallet.signIn(); };
document.querySelector('#sign-out-button').onclick = () => { wallet.signOut(); };

async function populateMenu() {
  loader();

  const populate = await wallet.callMethod({ method: 'populate_menu', contractId: CONTRACT_ADDRESS });
  getAvailableCoffees();

  loader("off");

  document.querySelector('#signed-in-flow main').classList.remove('please-wait');
  document.querySelector('#populate_menu').style.display = 'none';
}

async function initializeNft() {
  loader();
  const init_nft = await wallet.callMethod({ method: 'initialize_nft_contract', contractId: CONTRACT_ADDRESS });
  loader("off");
}

async function mintNft() {
  loader();
  let deposit = parseNearAmount("1.0");
  let coffee_uuid = crypto.randomUUID();
  const mint_nft = await wallet.callMethod({ method: 'mint_test_coffee_nft', args: { coffee: "Espresso", deposit: deposit, id: coffee_uuid}, contractId: CONTRACT_ADDRESS, deposit: deposit});
  loader("off");
}

async function getAvailableCoffees() {
  const availableCoffees = await wallet.viewMethod({ method: 'get_available_coffees', contractId: CONTRACT_ADDRESS });

  if(availableCoffees.length !== 0) {

    let coffee_menu = document.getElementById("coffee_menu");
    coffee_menu.innerHTML = "";
    let update_coffee_select = document.getElementById("existing_coffee_name");
    update_coffee_select.innerHTML = "";

    let coffee_menu_first = document.createElement("option");
      coffee_menu_first.value = "0.0";
      coffee_menu_first.text = "Select coffee";
      coffee_menu.add(coffee_menu_first);

    let update_option_first = document.createElement("option");
      update_option_first.value = "";
      update_option_first.text = "Select coffee";
      update_coffee_select.add(update_option_first);

    availableCoffees.forEach((coffee) => {
      let coffee_name = coffee[0];
      let coffee_price = coffee[1];
      
      let option = document.createElement("option");
      option.value = coffee_price;
      option.text = coffee_name;
      coffee_menu.add(option);

      let update_option = document.createElement("option");
      update_option.value = coffee_price;
      update_option.text = coffee_name;
      update_coffee_select.add(update_option);

      var new_price_input_field = document.getElementById("updated_coffee_price");
      var firstLoadedCoffeeValue = update_coffee_select.value;
      new_price_input_field.value.value = firstLoadedCoffeeValue;

      update_coffee_select.addEventListener('change', function handleChange(event) {
        new_price_input_field.value = event.target.value
      });

      let price_info = document.getElementById("price_info");
      coffee_menu.addEventListener('change', function handleChange(event) {
        price_info.innerHTML = event.target.value + " N";
      });
    });

    document.querySelector('form#buy_coffee').style.display = "block";

    if(wallet.accountId == owner && document.querySelector('#coffee_menu').options.length){
      document.querySelector('#manage_coffee').style.display = "block";
      document.querySelector('#populate_menu_button').style.display = "block";
      
    } else {
      document.querySelector('#manage_coffee').style.display = "none";
      document.querySelector('#populate_menu_button').style.display = "none";
    }
  } else {
    document.querySelector('#populate_menu').style.display = "block";
    document.querySelector('#manage_coffee').style.display = "none";
    if(wallet.accountId == owner) {
      document.querySelector('#populate_menu_button').style.display = "block";
    }
    document.querySelector('form#buy_coffee').style.display = "none";
    
  }
}

async function addCoffee(event) {

  event.preventDefault();
  let new_coffee_name = document.querySelector('#new_coffee_name').value;
  let new_coffee_price = document.querySelector('#new_coffee_price').value;
  let new_coffee_image = document.querySelector('#new_coffee_img_url').value;

  if(new_coffee_name == "" || new_coffee_price == "" || new_coffee_image == "") {
    alert("You must supply all data");
  } else {
    loader();

    const addNewCoffee = await wallet.callMethod({ method: 'add_new_coffee', args: { name: new_coffee_name, price: new_coffee_price, img_url: new_coffee_image}, contractId: CONTRACT_ADDRESS });

    getAvailableCoffees();

    loader("off");  }
}

async function updatePrice(event) {

  event.preventDefault();
  let selected_coffee = document.querySelector('#existing_coffee_name');
  let existing_coffee_name = selected_coffee.options[selected_coffee.selectedIndex].text;
  let updated_coffee_price = document.querySelector('#updated_coffee_price').value;

  if(existing_coffee_name == ""){
    alert("You must select a coffee");
  } else {
    loader();

    const updateCoffeePrice = await wallet.callMethod({ method: 'update_price', args: { name: existing_coffee_name, price: updated_coffee_price}, contractId: CONTRACT_ADDRESS });

    getAvailableCoffees();

    loader("off");
  }
}

async function getTotalSpent() {
  const totalSpent = await wallet.viewMethod({ method: 'get_patron_spent_amount', args: { account_id: wallet.accountId }, contractId: CONTRACT_ADDRESS });
  document.querySelector('[data-behavior=total-spent]').innerHTML = totalSpent;
  handleSpentDataUiElements(totalSpent);
}

async function buyCoffee(event) {

  event.preventDefault();
  let coffee_menu_select = document.querySelector('#coffee_menu');
  let existing_coffee_name = coffee_menu_select.options[coffee_menu_select.selectedIndex].text;
  let existing_coffee_price = coffee_menu_select.options[coffee_menu_select.selectedIndex].value;

  if(existing_coffee_name == "" || existing_coffee_price == "" || existing_coffee_price == "0.0"){
    alert("You must select a coffee");
  } else {

    loader();

    let coffee_uuid = crypto.randomUUID();
    let near_amount = parseNearAmount(existing_coffee_price);
    const buyCoffee = await wallet.callMethod({ method: 'buy_coffee', args: { name: existing_coffee_name, id: coffee_uuid }, contractId: CONTRACT_ADDRESS, deposit: near_amount});
  
    loader("off");
  }
}



function selectElement(id, valueToSelect) {    
  let element = document.getElementById(id);
  element.value = valueToSelect;
}


// UI: Display the signed-out-flow container
function signedOutFlow() {
  document.querySelector('#signed-in-flow').style.display = 'none';
  document.querySelector('#signed-out-flow').style.display = 'block';
  document.querySelector('.joined-as').style.display = 'none';
}

// UI: Displaying the signed in flow container and fill in account-specific data
function signedInFlow() {
  document.querySelector('#signed-out-flow').style.display = 'none';
  document.querySelector('#signed-in-flow').style.display = 'block';
  document.querySelectorAll('[data-behavior=account-id]').forEach(el => {
    el.innerText = wallet.accountId;
    el.dataset.text = wallet.accountId;
  });
  document.querySelector('.joined-as').style.display = 'block';
  getAvailableCoffees();
  getTotalSpent();
}


function handleSpentDataUiElements(totalSpent) {

  if (totalSpent == 0) {
    document.querySelector('.not-spent').style.display = "block";
    document.querySelector('.has-spent').style.display = "none";
  } else {
    document.querySelector('.not-spent').style.display = "none";
    document.querySelector('.has-spent').style.display = "block";
  }

  if(wallet.accountId == owner) {
    document.querySelector('.not-spent').style.display = "none";
    document.querySelector('.has-spent').style.display = "none";
  }
}

function loader(opt) {
  let loader_el = document.querySelector('.logo-loader');
  if (opt == "off") {
    loader_el.style.display = "none";
  } else {
    loader_el.style.display = "block";
  }
}
