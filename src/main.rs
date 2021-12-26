use yew::prelude::*;

enum Msg {
    Begin,  AddOne, SubOne, Dance
}

struct Model {
    value: i64,
    val2: String,
    image1: String,
    has_hp: bool,
    is_dancing: String,
    
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            val2: "Sweet Tardi-sons".to_owned(),
            image1: "images/tardigradebase.png".to_owned(),
            has_hp: false,
            is_dancing: "not-dancing".to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {

            Msg::Begin => {
                self.value += 1;
                self.val2 = "base tardigrade".to_string();
                self.image1 = "images/tardigradebase.png".to_string();
                self.has_hp = false;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
           
            Msg::AddOne => {
                self.value += 1;
                self.val2 = "gaming tardigrade".to_string();
                self.image1 = "images/redhp.png".to_string();
                self.has_hp = true;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }

            Msg::SubOne => {
                self.value -= 1;
                self.val2 = "Degen-tard".to_string();
                if self.has_hp{
                    self.image1 = String::from("images/pillowhptard.png");
                }
                else{
                    self.image1 = String::from("images/weebtard.png");

                }
                //self.image1 = String::from("images/weebtard.png");

                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }

            Msg::Dance=> {
                let tard_dance = String::from("tard-dance");
                if self.is_dancing.eq(&tard_dance){
                    self.is_dancing = String::from("not-dancing");
                }
                else{
                    self.is_dancing = tard_dance;
                }
                
                true
            }

           
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div style="display: flex; flex-direction: column; align-items: start"> //put into column and stick to left side of page
                <button onclick={link.callback(|_| Msg::Begin)}>{ "Reset" }</button>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "Headphones" }</button>
                //<button onclick={link.batch_callback(|_| vec![Msg::AddOne, Msg::AddOne, Msg::AddOne])}>{ "Begin" }</button>
                <button onclick={link.callback(|_| Msg::SubOne)}>{ "Pillow" }</button>
                <button onclick={link.callback(|_| Msg::Dance)}>{ "Dance" }</button>
                <p>{ self.value }</p>
                <p>{ self.val2.to_owned() }</p> //display self.val2
                <img style="width: 500px; height: 600px; margin: auto;" class={self.is_dancing.to_owned()} src={ self.image1.to_owned() }/> //display sweet walnut with the mash potatos or stacked tards
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}