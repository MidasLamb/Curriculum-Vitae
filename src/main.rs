#![recursion_limit = "1024"]
use yew::prelude::*;

mod data;

use data::Me;

const DATE_FORMAT: &str = "%m/%Y";

trait Htmlify {
    fn view(&self) -> Html;
}

struct Model {
    person: Me,
}

impl Component for Model {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            person: Me::default(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let contact_email_link = "mailto:".to_string()
            + self.person.contact_email
            + "?subject=You're hired!&body=We saw your CV and want to hire you immediately!";

        let person_links = self.person.links.view();
        let work_experience = self.person.work_experiences.view();
        let scholary_experience = self.person.scholary_experiences.view();
        let languages = self.person.languages.view();
        let skills = self.person.skills.view();
        let projects = self.person.projects.view();

        html! {
            <>
                <div class="page">
                    <div class="title">
                        <span class="cv">{"Curriculum Vitae: "}</span><span class="name">{self.person.first_name.to_owned() + " " + self.person.last_name}</span>
                    </div>
                    <hr/>
                    <div class="person-details">
                        <div class="details">
                            <table>
                                <tr>
                                    <td class="header">{"Full name: "}</td>
                                    <td class="name">{self.person.first_name.to_owned() + " " + self.person.last_name}</td>
                                </tr>
                                <tr>
                                    <td class="header">{"Email: "}</td>
                                    <td class="name"><a href=contact_email_link target="_blank">{self.person.contact_email}</a></td>
                                </tr>
                            </table>
                        </div>
                        <div class="links">
                            {person_links}
                        </div>
                    </div>
                    <hr/>
                    <main>
                        <div class="experience work">
                            <h2>{"Work Experience"}</h2>
                            <div class="container">
                                {work_experience}
                            </div>
                        </div>
                        <hr/>
                        <div class="experience education">
                            <h2>{"Education"}</h2>
                            <div class="container">
                                {scholary_experience}
                            </div>
                        </div>
                        <hr/>
                        <div class="languages">
                            <h2>{"Languages"}</h2>
                            <div class="container">
                                {languages}
                            </div>
                        </div>
                        <hr/>
                        <div class="skills">
                            <h2>{"Skills"}</h2>
                            <div class="container">
                                {skills}
                            </div>
                        </div>
                    </main>
                </div>
                <div class="page">
                    <main>
                        <div class="projects">
                            <h2>{"Projects"}</h2>
                            <div class="container">
                                {projects}
                            </div>
                        </div>
                    </main>
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

impl Htmlify for data::PersonLink {
    fn view(&self) -> Html {
        let icon = match self {
            data::PersonLink::GitHub { username } => {
                html! {<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 496 512"><path d="M165.9 397.4c0 2-2.3 3.6-5.2 3.6-3.3.3-5.6-1.3-5.6-3.6 0-2 2.3-3.6 5.2-3.6 3-.3 5.6 1.3 5.6 3.6zm-31.1-4.5c-.7 2 1.3 4.3 4.3 4.9 2.6 1 5.6 0 6.2-2s-1.3-4.3-4.3-5.2c-2.6-.7-5.5.3-6.2 2.3zm44.2-1.7c-2.9.7-4.9 2.6-4.6 4.9.3 2 2.9 3.3 5.9 2.6 2.9-.7 4.9-2.6 4.6-4.6-.3-1.9-3-3.2-5.9-2.9zM244.8 8C106.1 8 0 113.3 0 252c0 110.9 69.8 205.8 169.5 239.2 12.8 2.3 17.3-5.6 17.3-12.1 0-6.2-.3-40.4-.3-61.4 0 0-70 15-84.7-29.8 0 0-11.4-29.1-27.8-36.6 0 0-22.9-15.7 1.6-15.4 0 0 24.9 2 38.6 25.8 21.9 38.6 58.6 27.5 72.9 20.9 2.3-16 8.8-27.1 16-33.7-55.9-6.2-112.3-14.3-112.3-110.5 0-27.5 7.6-41.3 23.6-58.9-2.6-6.5-11.1-33.3 2.6-67.9 20.9-6.5 69 27 69 27 20-5.6 41.5-8.5 62.8-8.5s42.8 2.9 62.8 8.5c0 0 48.1-33.6 69-27 13.7 34.7 5.2 61.4 2.6 67.9 16 17.7 25.8 31.5 25.8 58.9 0 96.5-58.9 104.2-114.8 110.5 9.2 7.9 17 22.9 17 46.4 0 33.7-.3 75.4-.3 83.6 0 6.5 4.6 14.4 17.3 12.1C428.2 457.8 496 362.9 496 252 496 113.3 383.5 8 244.8 8zM97.2 352.9c-1.3 1-1 3.3.7 5.2 1.6 1.6 3.9 2.3 5.2 1 1.3-1 1-3.3-.7-5.2-1.6-1.6-3.9-2.3-5.2-1zm-10.8-8.1c-.7 1.3.3 2.9 2.3 3.9 1.6 1 3.6.7 4.3-.7.7-1.3-.3-2.9-2.3-3.9-2-.6-3.6-.3-4.3.7zm32.4 35.6c-1.6 1.3-1 4.3 1.3 6.2 2.3 2.3 5.2 2.6 6.5 1 1.3-1.3.7-4.3-1.3-6.2-2.2-2.3-5.2-2.6-6.5-1zm-11.4-14.7c-1.6 1-1.6 3.6 0 5.9 1.6 2.3 4.3 3.3 5.6 2.3 1.6-1.3 1.6-3.9 0-6.2-1.4-2.3-4-3.3-5.6-2z"/></svg>}
            }
            data::PersonLink::LinkedIn { link_name } => {
                html! {<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M416 32H31.9C14.3 32 0 46.5 0 64.3v383.4C0 465.5 14.3 480 31.9 480H416c17.6 0 32-14.5 32-32.3V64.3c0-17.8-14.4-32.3-32-32.3zM135.4 416H69V202.2h66.5V416zm-33.2-243c-21.3 0-38.5-17.3-38.5-38.5S80.9 96 102.2 96c21.2 0 38.5 17.3 38.5 38.5 0 21.3-17.2 38.5-38.5 38.5zm282.1 243h-66.4V312c0-24.8-.5-56.7-34.5-56.7-34.6 0-39.9 27-39.9 54.9V416h-66.4V202.2h63.7v29.2h.9c8.9-16.8 30.6-34.5 62.9-34.5 67.2 0 79.7 44.3 79.7 101.9V416z"/></svg>}
            }
        };

        html! {
            <div>
                <a href=self.linkify()>{icon}{self.short_view()}</a>
            </div>
        }
    }
}

impl Htmlify for data::WorkExperience {
    fn view(&self) -> Html {
        let start_date = format!("{}", self.start_date.format(DATE_FORMAT));
        let end_date = self.end_date.map_or("Present".to_owned(), |x| {
            format!("{}", x.format(DATE_FORMAT))
        });
        html! {
            <div class="experience-container">
                <div class="company">
                    <span class="header">{"Company: "}</span>
                    <span class="value">{self.company}</span>
                </div>
                <div class="job-title">
                    <span class="header">{"Job title: "}</span>
                    <span class="value">{self.title}</span>
                </div>
                <div class="summary">
                    {self.summary}
                </div>
                <div class="date">
                    {start_date}{" - "}{end_date}
                </div>
            </div>
        }
    }
}

impl Htmlify for data::ScholaryExperience {
    fn view(&self) -> Html {
        let start_date = format!("{}", self.start_date.format(DATE_FORMAT));
        let end_date = self.end_date.map_or("Present".to_owned(), |x| {
            format!("{}", x.format(DATE_FORMAT))
        });
        html! {
            <div class="education-container">
                <div class="degree">
                    <span class="value">{self.degree}</span>
                    <span class="honors">
                        {self.honors.unwrap()}
                    </span>
                </div>
                <div class="institution">
                    <span class="header">{"Institution: "}</span>
                    <span class="value">{self.institution}</span>
                </div>
                {
                    if self.thesis_title.is_some() {
                        html!{
                            <div class="thesis">
                                <span class="header">{"Thesis: "}</span>
                                <span class="value">{self.thesis_title.unwrap()}</span>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                <div class="date">
                    {start_date}{" - "}{end_date}
                </div>
            </div>
        }
    }
}

impl Htmlify for data::Language {
    fn view(&self) -> Html {
        html! {
            <div class="language">
                <div class="lang">{self.language}</div>
                <div class="proficiency">{self.proficiency.to_string()}</div>
            </div>
        }
    }
}

impl Htmlify for data::Skill {
    fn view(&self) -> Html {
        let note = self.note.as_ref().map_or(
            html! {},
            |n| html! {<span class="note">{n.to_string()}</span>},
        );
        let mut ability = vec![];
        for i in (0..10).step_by(2) {
            if i + 1 == self.ability {
                ability.push(html! {<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 536 512"><path d="M508.55 171.51L362.18 150.2 296.77 17.81C290.89 5.98 279.42 0 267.95 0c-11.4 0-22.79 5.9-28.69 17.81l-65.43 132.38-146.38 21.29c-26.25 3.8-36.77 36.09-17.74 54.59l105.89 103-25.06 145.48C86.98 495.33 103.57 512 122.15 512c4.93 0 10-1.17 14.87-3.75l130.95-68.68 130.94 68.7c4.86 2.55 9.92 3.71 14.83 3.71 18.6 0 35.22-16.61 31.66-37.4l-25.03-145.49 105.91-102.98c19.04-18.5 8.52-50.8-17.73-54.6zm-121.74 123.2l-18.12 17.62 4.28 24.88 19.52 113.45-102.13-53.59-22.38-11.74.03-317.19 51.03 103.29 11.18 22.63 25.01 3.64 114.23 16.63-82.65 80.38z"/></svg>});
            } else if i < self.ability {
                ability.push(html! {<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M259.3 17.8L194 150.2 47.9 171.5c-26.2 3.8-36.7 36.1-17.7 54.6l105.7 103-25 145.5c-4.5 26.3 23.2 46 46.4 33.7L288 439.6l130.7 68.7c23.2 12.2 50.9-7.4 46.4-33.7l-25-145.5 105.7-103c19-18.5 8.5-50.8-17.7-54.6L382 150.2 316.7 17.8c-11.7-23.6-45.6-23.9-57.4 0z"/></svg>});
            } else {
                ability.push(html! {<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M528.1 171.5L382 150.2 316.7 17.8c-11.7-23.6-45.6-23.9-57.4 0L194 150.2 47.9 171.5c-26.2 3.8-36.7 36.1-17.7 54.6l105.7 103-25 145.5c-4.5 26.3 23.2 46 46.4 33.7L288 439.6l130.7 68.7c23.2 12.2 50.9-7.4 46.4-33.7l-25-145.5 105.7-103c19-18.5 8.5-50.8-17.7-54.6zM388.6 312.3l23.7 138.4L288 385.4l-124.3 65.3 23.7-138.4-100.6-98 139-20.2 62.2-126 62.2 126 139 20.2-100.6 98z"/></svg>});
            }
        }

        html! {
            <div class="skill-container">
                <div class="skill">
                    <span class="name">{self.name}</span>
                    {note}
                </div>
                <div class="ability">
                    {ability}
                </div>
            </div>
        }
    }
}

impl Htmlify for data::Project {
    fn view(&self) -> Html {
        let technologies = self
            .used_technologies
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        html! {
            <div class="project-container">
                <div class="project-header">
                    <div class="project-name">{self.name}</div>
                    {
                        if self.link.is_some() {
                            html!{
                                <a href=self.link.unwrap() class="project-link" target="_blank">{"Link"}</a>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
                <div class="used-technologies">{technologies}</div>
                <div class="summary">
                    {self.summary}
                </div>
            </div>
        }
    }
}

impl<T> Htmlify for Vec<T>
where
    T: Htmlify,
{
    fn view(&self) -> Html {
        let v = self.iter().map(|x| x.view()).collect::<Vec<_>>();
        html! {<>{v}</>}
    }
}
