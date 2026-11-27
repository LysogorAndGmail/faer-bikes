use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="sticky top-0 z-50 bg-brand-dark/95 backdrop-blur-md border-b border-gray-800 text-white">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex justify-between items-center h-20">
                <!-- Логотип -->
                <a href="#" class="flex items-center space-x-3 group">
                    <div class="w-10 h-10 rounded-xl bg-brand-accent flex items-center justify-center text-white shadow-lg shadow-orange-500/20 group-hover:scale-105 transition-transform">
                        <!-- Иконка шестеренки / звезды велосипеда -->
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                            <circle cx="12" cy="12" r="3"></circle>
                            <path stroke-linecap="round" stroke-linejoin="round" d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-2 2 2 2 0 01-2-2v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06a1.65 1.65 0 00.33-1.82 1.65 1.65 0 00-1.51-1H3a2 2 0 01-2-2 2 2 0 012-2h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06a1.65 1.65 0 001.82.33H9a1.65 1.65 0 001-1.51V3a2 2 0 012-2 2 2 0 012 2v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06a1.65 1.65 0 00-.33 1.82V9a1.65 1.65 0 001.51 1H21a2 2 0 012 2 2 2 0 01-2 2h-.09a1.65 1.65 0 00-1.51 1z"></path>
                        </svg>
                    </div>
                    <div>
                        <span class="font-display font-extrabold text-xl tracking-tight">VELO<span class="text-brand-accent">MASTER</span></span>
                        <span class="block text-[11px] text-gray-400 uppercase tracking-widest -mt-1">Bike Service Lab</span>
                    </div>
                </a>

                <!-- Десктопное меню -->
                <nav class="hidden md:flex items-center space-x-8 text-sm font-medium text-gray-300">
                    <a href="#services" class="hover:text-white transition-colors">Услуги</a>
                    <a href="#pricing" class="hover:text-white transition-colors">Прайс и калькулятор</a>
                    <a href="#features" class="hover:text-white transition-colors">О мастерской</a>
                    <a href="#contacts" class="hover:text-white transition-colors">Контакты</a>
                </nav>

                <!-- Кнопка записи и бургер -->
                <div class="flex items-center space-x-3">
                    <a href="#booking" class="hidden sm:inline-flex items-center px-5 py-2.5 rounded-xl bg-brand-accent hover:bg-brand-accentHover text-white font-semibold text-sm transition-all shadow-lg shadow-orange-500/25">
                        Записаться
                    </a>
                    <button id="mobile-menu-btn" aria-label="Открыть меню" class="md:hidden p-2.5 rounded-xl text-gray-300 hover:text-white hover:bg-gray-800 transition-colors">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16"></path>
                        </svg>
                    </button>
                </div>
            </div>
        </div>

        <!-- Мобильное меню -->
        <div id="mobile-menu" class="hidden md:hidden bg-gray-900 border-b border-gray-800 px-4 pt-2 pb-6 space-y-2">
            <a href="#services" class="mobile-link block px-4 py-3 rounded-xl text-gray-300 hover:bg-gray-800 hover:text-white font-medium">Услуги</a>
            <a href="#pricing" class="mobile-link block px-4 py-3 rounded-xl text-gray-300 hover:bg-gray-800 hover:text-white font-medium">Прайс и калькулятор</a>
            <a href="#features" class="mobile-link block px-4 py-3 rounded-xl text-gray-300 hover:bg-gray-800 hover:text-white font-medium">О мастерской</a>
            <a href="#contacts" class="mobile-link block px-4 py-3 rounded-xl text-gray-300 hover:bg-gray-800 hover:text-white font-medium">Контакты</a>
            <div class="pt-2">
                <a href="#booking" class="mobile-link block w-full text-center px-5 py-3 rounded-xl bg-brand-accent text-white font-semibold">
                    Записаться на ремонт
                </a>
            </div>
        </div>
    </header>

    }
}
