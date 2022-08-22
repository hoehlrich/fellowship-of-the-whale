use yew::prelude::*;

use crate::components::team::Team;

pub struct Teams;
impl Component for Teams {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container is-max-desktop pb-5">
                <div class="tile is-child hero">
                    <div class="hero-body container pb-auto">
                        <h1 class="title is-1">{ "Teams" }</h1>
                    </div>
                </div>
                <div class="columns is-variable is-1-mobile is-0-tablet is-3-desktop is-8-widescreen is-2-fullhd">
                    <Team
                        team="Design"
                        description="Design Team leads the team in prototyping, allowing different mechanisms and ideas to be built and tested out. After determining which prototypes work the best, the Design Team creates a computer-aided design (CAD) of the robot before it is built."
                        img_src="../../static/img/team_icons/design.png"
                    />
                    <Team
                        team="Build"
                        description="The Build Team takes a design and makes it a reality.  In addition to machining, Build Team members learn about topics such as robot wiring and pneumatics systems, bringing all of this knowledge to help create the final product."
                        img_src="../../static/img/team_icons/build.png"
                    />
                    <Team
                        team="Software"
                        description="Writing code in Rust, the Software Team (henry) works alongside the Build Team to test our robot and make it functional. Ever haunted by the phrase: \"They\'ll fix it in the software.\""
                        img_src="../../static/img/team_icons/software.png"
                    />
                </div>
                <div class="columns is-variable is-1-mobile is-0-tablet is-3-desktop is-8-widescreen is-2-fullhd">
                   <Team
                        team="Art"
                        description="FIRST also holds an annual animation contest! Our Art Team works together to create short videos for this competition using Autodesk 3D animation software. Art Team also designs our t-shirts, posters, flyers, banners, logos, and buttons."
                        img_src="../../static/img/team_icons/art.png"
                    />
                    <Team
                        team="Strategy and Scouting"
                        description="The Strategy and Scouting Team is in charge of creating a web-based system for the team to use while scouting and planning out matches at competition. They are also responsible for helping determine the objectives of the robot at the beginning of each season."
                        img_src="../../static/img/team_icons/ss.png"
                    />
                    <Team
                        team="Business"
                        description="The Business Team oversees all corporate sponsorships, fundraising, outreach, awards, and public relations efforts. They develop business, sponsorship, and diversity plans."
                        img_src="../../static/img/team_icons/business.png"
                    />
                </div>
            </div>
        }
    }
}
