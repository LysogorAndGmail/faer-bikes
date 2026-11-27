use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, StaticSegment};
use crate::components::{Header, Home, Footer, SuccessModal}; // Импортируем ваши компоненты


pub fn shell(options: LeptosOptions) -> impl IntoView {
   let tailwind_config = r#"
        tailwind.config = {
            theme: {
                extend: {
                    fontFamily: {
                        sans: ['Inter', 'sans-serif'],
                        display: ['Manrope', 'sans-serif'],
                    },
                    colors: {
                        brand: {
                            dark: '#111827',
                            card: '#1F2937',
                            accent: '#F97316',
                            accentHover: '#EA580C',
                            light: '#F9FAFB'
                        }
                    }
                }
            }
        }
    "#;
    view! {
        <!DOCTYPE html>
        <html lang="en" class="scroll-smooth" >
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>

                <script src="https://cdn.tailwindcss.com"></script>
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
		<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Manrope:wght@700;800&display=swap" rel="stylesheet"/>
                <script inner_html=tailwind_config></script>
            </head>
            <body class="bg-brand-light text-gray-900 font-sans antialiased selection:bg-brand-accent selection:text-white" >
                <App/>
                <script src="/js/site.js"></script>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/faer-bikes.css"/>
        <Title text="Faer Bikes"/>

        <Router>
            <Header/>
            
             <main>
                <Routes fallback=|| "Страница не найдена.".into_view()>
                    <Route path=StaticSegment("") view=Home/>
                    // Сюда можно добавлять новые страницы:
                    // <Route path=StaticSegment("catalog") view=CatalogPage/>
                </Routes>
            </main>
            <SuccessModal/>
            <Footer/>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <section class="hero">
            <h1>"Добро пожаловать в Faer Bikes"</h1>
            // Сюда вставляете основной контент главной страницы из index.html
        </section>
    }
}
