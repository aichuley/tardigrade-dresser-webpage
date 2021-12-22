use yew::prelude::*;

enum Msg {
    AddOne, SubOne
}

struct Model {
    value: i64,
    val2: String,
    image1: String,
    
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            val2: "Sweet Tardi-sons".to_owned(),
            image1: "".to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                self.val2 = "happy son-digrade".to_string();
                self.image1 = "images/walmash.jpg".to_string();
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }

            Msg::SubOne => {
                self.value -= 1;
                self.val2 = "2 stacked son-digrades".to_string();
                self.image1 = String::from("images/sweettards.jpg");

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
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <button onclick={link.batch_callback(|_| vec![Msg::AddOne, Msg::AddOne, Msg::AddOne])}>{ "+1 +1 +1" }</button>
                <button onclick={link.callback(|_| Msg::SubOne)}>{ "-1" }</button>
                <p>{ self.value }</p>
                <p>{ self.val2.to_owned() }</p> //display self.val2
                <img style="width: 500px; height: 600px;" src={ self.image1.to_owned() }/> //display sweet walnut with the mash potatos or stacked tards
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}