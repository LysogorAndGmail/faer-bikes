use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
            <footer id="contacts" class="bg-brand-dark text-gray-300 border-t border-gray-800 mt-12">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-14">
            <div class="grid md:grid-cols-2 lg:grid-cols-12 gap-10 items-start">
                <!-- Информация о мастерской -->
                <div class="lg:col-span-5 space-y-4">
                    <div class="flex items-center space-x-3">
                        <div class="w-9 h-9 rounded-xl bg-brand-accent flex items-center justify-center text-white font-bold">V</div>
                        <span class="font-display font-extrabold text-xl text-white">VELO<span class="text-brand-accent">MASTER</span></span>
                    </div>
                    <p class="text-sm text-gray-400 leading-relaxed">
                        Современная веломастерская с удобным заездом прямо с велодорожки. Пока мастер настраивает ваш велосипед, угостим свежесваренным кофе.
                    </p>
                    <div class="space-y-2.5 pt-2 text-sm">
                        <div class="flex items-start space-x-3">
                            <svg class="w-5 h-5 text-brand-accent shrink-0 mt-0.5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"></path>
                            </svg>
                            <span>ул. Велосипедная, д. 14, корп. 2 (вход со стороны парка)</span>
                        </div>
                        <div class="flex items-center space-x-3">
                            <svg class="w-5 h-5 text-brand-accent shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                            <span>Ежедневно: 10:00 - 21:00 без выходных</span>
                        </div>
                        <div class="flex items-center space-x-3">
                            <svg class="w-5 h-5 text-brand-accent shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"></path>
                            </svg>
                            <a href="tel:+79990001122" class="text-white font-semibold hover:text-brand-accent transition-colors">+7 (999) 000-11-22</a>
                        </div>
                    </div>
                </div>

                <!-- Быстрые ссылки -->
                <div class="lg:col-span-3 space-y-3 text-sm">
                    <h4 class="font-display font-bold text-white uppercase tracking-wider text-xs">Навигация</h4>
                    <ul class="space-y-2 text-gray-400">
                        <li><a href="#services" class="hover:text-white transition-colors">Услуги мастерской</a></li>
                        <li><a href="#pricing" class="hover:text-white transition-colors">Прайс-лист и калькулятор</a></li>
                        <li><a href="#features" class="hover:text-white transition-colors">Преимущества</a></li>
                        <li><a href="#booking" class="hover:text-white transition-colors">Онлайн-запись</a></li>
                    </ul>
                </div>

                <!-- Блок карты (заглушка под Яндекс/Google Карты) -->
                <div class="lg:col-span-4">
                    <div class="rounded-2xl bg-gray-800 border border-gray-700 h-48 flex flex-col items-center justify-center p-6 text-center relative overflow-hidden">
                        <div class="absolute inset-0 bg-[radial-gradient(#4B5563_1px,transparent_1px)] [background-size:16px_16px] opacity-30"></div>
                        <div class="relative z-10">
                            <span class="inline-flex items-center justify-center w-10 h-10 rounded-full bg-brand-accent/20 text-brand-accent mb-2">
                                '📍'
                            </span>
                            <p class="text-sm font-semibold text-white">Схема проезда к мастерской</p>
                            <p class="text-xs text-gray-400 mt-1">Вставьте сюда iframe-код виджета Яндекс.Карт или 2ГИС</p>
                        </div>
                    </div>
                </div>
            </div>

            <div class="border-t border-gray-800 mt-12 pt-6 flex flex-col sm:flex-row justify-between items-center text-xs text-gray-500 gap-4">
                <p>'©' 2026 VeloMaster. Все права защищены.</p>
                <div class="flex space-x-5">
                    <a href="#" class="hover:text-gray-300 transition-colors">Telegram</a>
                    <a href="#" class="hover:text-gray-300 transition-colors">WhatsApp</a>
                    <a href="#" class="hover:text-gray-300 transition-colors">ВКонтакте</a>
                </div>
            </div>
        </div>
    </footer>

    }
}
