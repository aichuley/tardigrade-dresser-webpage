use yew::prelude::*;

enum Msg {
    Begin,  AddOne, SubOne
}

struct Model {
    value: i64,
    val2: String,
    image1: String,
    has_hp: bool,
    
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
                <p>{ self.value }</p>
                <p>{ self.val2.to_owned() }</p> //display self.val2
                <img style="width: 500px; height: 600px; margin: auto;" src={ self.image1.to_owned() }/> //display sweet walnut with the mash potatos or stacked tards
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}