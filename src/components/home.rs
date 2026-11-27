use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <section class="relative bg-brand-dark text-white overflow-hidden">
        // Декоративная сетка и свечение 
        <div class="absolute inset-0 bg-[radial-gradient(#374151_1px,transparent_1px)] [background-size:24px_24px] opacity-25"/>
        <div class="absolute -top-24 -right-24 w-96 h-96 bg-brand-accent/20 rounded-full blur-3xl pointer-events-none"/>

        <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16 sm:py-24 lg:py-28">
            <div class="grid lg:grid-cols-12 gap-12 items-center">
                <div class="lg:col-span-7 space-y-6">
                    // Бейдж режима работы -->
                    <div class="inline-flex items-center space-x-2 px-3.5 py-1.5 rounded-full bg-gray-800/90 border border-gray-700 text-xs sm:text-sm text-gray-300">
                        <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"/>
                        <span>Открыты ежедневно с 10:00 до 21:00 - Бесплатная диагностика</span>
                    </div>

                    <h1 class="font-display font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight leading-tight">
                        Вернём вашему байку <span class="text-brand-accent">идеальный накат</span> и чёткость работы
                    </h1>

                    <p class="text-base sm:text-lg text-gray-400 max-w-2xl leading-relaxed">
                        Профессиональный ремонт и сезонное ТО горных, шоссейных, гравийных и электровелосипедов. Работаем профессиональным инструментом Park Tool с гарантией на все узлы.
                    </p>

                    <div class="flex flex-col sm:flex-row gap-4 pt-2">
                        <a href="#booking" class="inline-flex justify-center items-center px-7 py-4 rounded-xl bg-brand-accent hover:bg-brand-accentHover text-white font-semibold text-base transition-all shadow-lg shadow-orange-500/25">
                            Записаться на сервис
                            <svg class="w-5 h-5 ml-2" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M13 7l5 5m0 0l-5 5m5-5H6"/>
                            </svg>
                        </a>
                        <a href="#pricing" class="inline-flex justify-center items-center px-7 py-4 rounded-xl bg-gray-800 hover:bg-gray-700 text-white border border-gray-700 font-semibold text-base transition-all">
                            Рассчитать стоимость
                        </a>
                    </div>

                    // Мини-статистика -->
                    <div class="grid grid-cols-3 gap-4 pt-6 border-t border-gray-800 max-w-lg">
                        <div>
                            <div class="font-display font-extrabold text-2xl sm:text-3xl text-white">8+ лет</div>
                            <div class="text-xs sm:text-sm text-gray-400">опыта механиков</div>
                        </div>
                        <div>
                            <div class="font-display font-extrabold text-2xl sm:text-3xl text-white">24 часа</div>
                            <div class="text-xs sm:text-sm text-gray-400">средний срок ТО</div>
                        </div>
                        <div>
                            <div class="font-display font-extrabold text-2xl sm:text-3xl text-white">30 дней</div>
                            <div class="text-xs sm:text-sm text-gray-400">гарантия на работы</div>
                        </div>
                    </div>
                </div>

                // Правая карточка / визуальный блок -->
                <div class="lg:col-span-5">
                    <div class="relative rounded-2xl overflow-hidden border border-gray-800 bg-brand-card shadow-2xl">
                        <img 
                            src="https://images.unsplash.com/photo-1532298229144-0ec0c57515c7?auto=format&fit=crop&w=900&q=80" 
                            onerror="this.src='https://placehold.co/800x600/1F2937/F97316?text=VeloMaster+Workshop'"
                            alt="Ремонт велосипеда в мастерской" 
                            class="w-full h-72 sm:h-96 object-cover opacity-90"
                        />
                        <div class="p-6 bg-gradient-to-t from-gray-950 via-gray-900 to-gray-900/95">
                            <div class="flex items-center justify-between">
                                <div>
                                    <span class="text-xs font-semibold uppercase tracking-wider text-brand-accent">Хит сезона</span>
                                    <h3 class="font-display font-bold text-lg text-white mt-0.5">Комплексное ТО "Полный перебор"</h3>
                                </div>
                                <div class="text-right">
                                    <span class="block text-xs text-gray-400 line-through">4 500 UAH</span>
                                    <span class="font-display font-extrabold text-xl text-white">3 490 UAH</span>
                                </div>
                            </div>
                            <p class="text-xs text-gray-400 mt-2">
                                Полная разборка, ультразвуковая мойка трансмиссии, переборка втулок и рулевой, правка колес и настройка тормозов.
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    // ==================== НАПРАВЛЕНИЯ УСЛУГ ==================== -->
    <section id="services" class="py-16 sm:py-24 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="text-center max-w-2xl mx-auto mb-12 sm:mb-16">
            <span class="text-brand-accent font-semibold text-sm uppercase tracking-wider">Что мы делаем</span>
            <h2 class="font-display font-extrabold text-3xl sm:text-4xl text-gray-900 mt-2">Полный спектр велосервиса</h2>
            <p class="text-gray-600 mt-3 text-base">От быстрой подкачки колес и настройки переключателя до сложного ремонта карбоновых рам и амортизаторов.</p>
        </div>

        <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-6 sm:gap-8">
            // Карточка 1 -->
            <div class="bg-white rounded-2xl p-7 border border-gray-200 shadow-sm hover:shadow-xl hover:border-brand-accent/40 transition-all group">
                <div class="w-12 h-12 rounded-xl bg-orange-50 text-brand-accent flex items-center justify-center mb-5 group-hover:bg-brand-accent group-hover:text-white transition-colors">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                        <circle cx="12" cy="12" r="3"/>
                    </svg>
                </div>
                <h3 class="font-display font-bold text-xl text-gray-900 mb-2">Сезонное и полное ТО</h3>
                <p class="text-gray-600 text-sm leading-relaxed">Комплексная подготовка велосипеда к сезону или консервация на зиму. Мойка, смазка всех подшипников и протяжка резьбовых соединений.</p>
            </div>

            // Карточка 2 -->
            <div class="bg-white rounded-2xl p-7 border border-gray-200 shadow-sm hover:shadow-xl hover:border-brand-accent/40 transition-all group">
                <div class="w-12 h-12 rounded-xl bg-orange-50 text-brand-accent flex items-center justify-center mb-5 group-hover:bg-brand-accent group-hover:text-white transition-colors">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                        <circle cx="12" cy="12" r="9"/>
                        <circle cx="12" cy="12" r="2"/>
                        <path stroke-linecap="round" d="M12 3v7m0 4v7M3 12h7m4 0h7"/>
                    </svg>
                </div>
                <h3 class="font-display font-bold text-xl text-gray-900 mb-2">Колёса и спицовка</h3>
                <p class="text-gray-600 text-sm leading-relaxed">Устранение "восьмёрок" и "яиц" на профессиональном станке, сборка колёс с нуля по тензометру, установка бескамерных покрышек (Tubeless).</p>
            </div>

            // Карточка 3 -->
            <div class="bg-white rounded-2xl p-7 border border-gray-200 shadow-sm hover:shadow-xl hover:border-brand-accent/40 transition-all group">
                <div class="w-12 h-12 rounded-xl bg-orange-50 text-brand-accent flex items-center justify-center mb-5 group-hover:bg-brand-accent group-hover:text-white transition-colors">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"/>
                    </svg>
                </div>
                <h3 class="font-display font-bold text-xl text-gray-900 mb-2">Тормоза и трансмиссия</h3>
                <p class="text-gray-600 text-sm leading-relaxed">Прокачка гидравлики Shimano, SRAM, Magura (DOT и минеральное масло), правка роторов и петуха, замена тросов, рубашек и цепей.</p>
            </div>

            // Карточка 4 -->
            <div class="bg-white rounded-2xl p-7 border border-gray-200 shadow-sm hover:shadow-xl hover:border-brand-accent/40 transition-all group">
                <div class="w-12 h-12 rounded-xl bg-orange-50 text-brand-accent flex items-center justify-center mb-5 group-hover:bg-brand-accent group-hover:text-white transition-colors">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M19 14l-7 7m0 0l-7-7m7 7V3"/>
                    </svg>
                </div>
                <h3 class="font-display font-bold text-xl text-gray-900 mb-2">Вилки и амортизаторы</h3>
                <p class="text-gray-600 text-sm leading-relaxed">Нулевое и полное ТО воздушных и пружинно-масляных вилок RockShox, Fox, Manitou, SR Suntour с заменой сальников и демпферного масла.</p>
            </div>

            // Карточка 5 -->
            <div class="bg-white rounded-2xl p-7 border border-gray-200 shadow-sm hover:shadow-xl hover:border-brand-accent/40 transition-all group">
                <div class="w-12 h-12 rounded-xl bg-orange-50 text-brand-accent flex items-center justify-center mb-5 group-hover:bg-brand-accent group-hover:text-white transition-colors">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                    </svg>
                </div>
                <h3 class="font-display font-bold text-xl text-gray-900 mb-2">Ремонт электровелосипедов</h3>
                <p class="text-gray-600 text-sm leading-relaxed">Диагностика мотор-колёс, кареточных моторов, контроллеров и проводки. Усиление дропаутов, обслуживание тормозной системы E-Bike.</p>
            </div>

            // Карточка 6 -->
            <div class="bg-white rounded-2xl p-7 border border-gray-200 shadow-sm hover:shadow-xl hover:border-brand-accent/40 transition-all group">
                <div class="w-12 h-12 rounded-xl bg-orange-50 text-brand-accent flex items-center justify-center mb-5 group-hover:bg-brand-accent group-hover:text-white transition-colors">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"/>
                    </svg>
                </div>
                <h3 class="font-display font-bold text-xl text-gray-900 mb-2">Сборка из коробки и кастом</h3>
                <p class="text-gray-600 text-sm leading-relaxed">Профессиональная сборка нового велосипеда из транспортной коробки, торцовка кареточного стакана, подбор и установка апгрейдов.</p>
            </div>
        </div>
    </section>

    // ==================== ПРАЙС-ЛИСТ И КАЛЬКУЛЯТОР ==================== -->
    <section id="pricing" class="py-16 sm:py-24 bg-gray-100 border-y border-gray-200">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex flex-col md:flex-row md:items-end justify-between mb-10 gap-4">
                <div>
                    <span class="text-brand-accent font-semibold text-sm uppercase tracking-wider">Прозрачные цены</span>
                    <h2 class="font-display font-extrabold text-3xl sm:text-4xl text-gray-900 mt-2">Прайс-лист и калькулятор ТО</h2>
                    <p class="text-gray-600 mt-2">Отметьте нужные услуги галочками, чтобы узнать предварительную стоимость ремонта.</p>
                </div>

                // Фильтр категорий -->
                <div class="flex flex-wrap gap-2" id="price-filters">
                    <button data-filter="all" class="filter-btn active px-4 py-2 rounded-xl text-sm font-semibold bg-brand-dark text-white transition-colors">Все услуги</button>
                    <button data-filter="to" class="filter-btn px-4 py-2 rounded-xl text-sm font-semibold bg-white text-gray-700 hover:bg-gray-200 transition-colors">Комплексное ТО</button>
                    <button data-filter="drivetrain" class="filter-btn px-4 py-2 rounded-xl text-sm font-semibold bg-white text-gray-700 hover:bg-gray-200 transition-colors">Трансмиссия и тормоза</button>
                    <button data-filter="wheels" class="filter-btn px-4 py-2 rounded-xl text-sm font-semibold bg-white text-gray-700 hover:bg-gray-200 transition-colors">Колёса и вилка</button>
                </div>
            </div>

            <div class="grid lg:grid-cols-12 gap-8 items-start">
                // Список услуг с чекбоксами -->
                <div class="lg:col-span-8 bg-white rounded-2xl border border-gray-200 shadow-sm overflow-hidden">
                    <div class="divide-y divide-gray-100" id="services-checklist">
                        // Позиция 1 -->
                        <label data-category="to" class="service-row flex items-center justify-between p-4 sm:p-5 hover:bg-orange-50/40 cursor-pointer transition-colors">
                            <div class="flex items-start space-x-3.5 pr-4">
                                <input type="checkbox" data-name="Базовое ТО (настройка + смазка)" data-price="1800" class="service-checkbox mt-1 w-5 h-5 rounded border-gray-300 text-brand-accent focus:ring-brand-accent"/>
                                <div>
                                    <span class="font-semibold text-gray-900 block">Базовое ТО</span>
                                    <span class="text-xs text-gray-500">Настройка переключателей и тормозов, протяжка узлов, смазка цепи</span>
                                </div>
                            </div>
                            <span class="font-display font-bold text-gray-900 whitespace-nowrap">1 800 UAH</span>
                        </label>

                        // Позиция 2 -->
                        <label data-category="to" class="service-row flex items-center justify-between p-4 sm:p-5 hover:bg-orange-50/40 cursor-pointer transition-colors">
                            <div class="flex items-start space-x-3.5 pr-4">
                                <input type="checkbox" data-name="Полное ТО «Под ключ»" data-price="3490" class="service-checkbox mt-1 w-5 h-5 rounded border-gray-300 text-brand-accent focus:ring-brand-accent"/>
                                <div>
                                    <span class="font-semibold text-gray-900 block">Полное ТО "Под ключ"</span>
                                    <span class="text-xs text-gray-500">Полная разборка, мойка трансмиссии, переборка втулок, каретки, рулевой, правка колёс</span>
                                </div>
                            </div>
                            <span class="font-display font-bold text-gray-900 whitespace-nowrap">3 490 UAH</span>
                        </label>

                        // Позиция 3 -->
                        <label data-category="drivetrain" class="service-row flex items-center justify-between p-4 sm:p-5 hover:bg-orange-50/40 cursor-pointer transition-colors">
                            <div class="flex items-start space-x-3.5 pr-4">
                                <input type="checkbox" data-name="Настройка переключателя (перед/зад)" data-price="500" class="service-checkbox mt-1 w-5 h-5 rounded border-gray-300 text-brand-accent focus:ring-brand-accent"/>
                                <div>
                                    <span class="font-semibold text-gray-900 block">Настройка переключателя (1 шт.)</span>
                                    <span class="text-xs text-gray-500">Регулировка ограничителей, натяжения троса и проверка геометрии петуха</span>
                                </div>
                            </div>
                            <span class="font-display font-bold text-gray-900 whitespace-nowrap">500 UAH</span>
                        </label>

                        // Позиция 4 -->
                        <label data-category="drivetrain" class="service-row flex items-center justify-between p-4 sm:p-5 hover:bg-orange-50/40 cursor-pointer transition-colors">
                            <div class="flex items-start space-x-3.5 pr-4">
                                <input type="checkbox" data-name="Прокачка гидравлического тормоза" data-price="900" class="service-checkbox mt-1 w-5 h-5 rounded border-gray-300 text-brand-accent focus:ring-brand-accent"/>
                                <div>
                                    <span class="font-semibold text-gray-900 block">Прокачка гидравлического тормоза (1 контур)</span>
                                    <span class="text-xs text-gray-500">Замена тормозной жидкости (DOT или Mineral Oil), удаление воздуха, центровка калипера</span>
                                </div>
                            </div>
                            <span class="font-display font-bold text-gray-900 whitespace-nowrap">900 UAH</span>
                        </label>

                        // Позиция 5 -->
                        <label data-category="drivetrain" class="service-row flex items-center justify-between p-4 sm:p-5 hover:bg-orange-50/40 cursor-pointer transition-colors">
                            <div class="flex items-start space-x-3.5 pr-4">
                                <input type="checkbox" data-name="Замена цепи и кассеты" data-price="700" class="service-checkbox mt-1 w-5 h-5 rounded border-gray-300 text-brand-accent focus:ring-brand-accent"/>
                                <div>
                                    <span class="font-semibold text-gray-900 block">Замена цепи и кассеты / трещотки</span>
                                    <span class="text-xs text-gray-500">Демонтаж старых компонентов, чистка барабана, установка и укорачивание цепи</span>
                                </div>
                            </div>
                            <span class="font-display font-bold text-gray-900 whitespace-nowrap">700 UAH</span>
                        </label>

                        // Позиция 6 -->
                        <label data-category="wheels" class="service-row flex items-center justify-between p-4 sm:p-5 hover:bg-orange-50/40 cursor-pointer transition-colors">
                            <div class="flex items-start space-x-3.5 pr-4">
                                <input type="checkbox" data-name="Исправление «восьмёрки» колеса" data-price="650" class="service-checkbox mt-1 w-5 h-5 rounded border-gray-300 text-brand-accent focus:ring-brand-accent"/>
                                <div>
                                    <span class="font-semibold text-gray-900 block">Исправление "восьмёрки" / натяжение спиц</span>
                                    <span class="text-xs text-gray-500">Правка геометрии обода на профессиональном станке с контролем зонта</span>
                                </div>
                            </div>
                            <span class="font-display font-bold text-gray-900 whitespace-nowrap">650 UAH</span>
                        </label>

                        // Позиция 7 -->
                        <label data-category="wheels" class="service-row flex items-center justify-between p-4 sm:p-5 hover:bg-orange-50/40 cursor-pointer transition-colors">
                            <div class="flex items-start space-x-3.5 pr-4">
                                <input type="checkbox" data-name="Переборка втулки колеса" data-price="850" class="service-checkbox mt-1 w-5 h-5 rounded border-gray-300 text-brand-accent focus:ring-brand-accent"/>
                                <div>
                                    <span class="font-semibold text-gray-900 block">Переборка втулки колеса</span>
                                    <span class="text-xs text-gray-500">Мойка чашек и конусов, закладка морской водостойкой смазки, регулировка люфта</span>
                                </div>
                            </div>
                            <span class="font-display font-bold text-gray-900 whitespace-nowrap">850 UAH</span>
                        </label>

                        // Позиция 8 -->
                        <label data-category="wheels" class="service-row flex items-center justify-between p-4 sm:p-5 hover:bg-orange-50/40 cursor-pointer transition-colors">
                            <div class="flex items-start space-x-3.5 pr-4">
                                <input type="checkbox" data-name="Нулевое ТО амортизационной вилки" data-price="1600" class="service-checkbox mt-1 w-5 h-5 rounded border-gray-300 text-brand-accent focus:ring-brand-accent"/>
                                <div>
                                    <span class="font-semibold text-gray-900 block">Нулевое ТО амортизационной вилки</span>
                                    <span class="text-xs text-gray-500">Снятие штанов, чистка пыльников, пропитка поролоновых колец и замена масла в штанах</span>
                                </div>
                            </div>
                            <span class="font-display font-bold text-gray-900 whitespace-nowrap">1 600 UAH</span>
                        </label>
                    </div>
                </div>

                // Блок итога калькулятора -->
                <div class="lg:col-span-4 bg-brand-dark text-white rounded-2xl p-6 sm:p-7 shadow-xl sticky top-28">
                    <h3 class="font-display font-bold text-xl flex items-center justify-between">
                        <span>Ваш расчёт</span>
                        <button id="clear-calc-btn" class="text-xs font-normal text-gray-400 hover:text-brand-accent underline hidden">Сбросить</button>
                    </h3>
                    
                    <div id="selected-list" class="mt-4 space-y-2.5 text-sm text-gray-300 min-h-[90px] border-y border-gray-800 py-4">
                        <p class="text-gray-500 italic">Выберите услуги в списке слева, чтобы рассчитать итоговую сумму...</p>
                    </div>

                    <div class="mt-5 flex items-baseline justify-between">
                        <span class="text-gray-400 text-sm">Ориентировочно:</span>
                        <span id="total-price" class="font-display font-extrabold text-3xl text-brand-accent">0 UAH</span>
                    </div>
                    <p class="text-[11px] text-gray-400 mt-1">* Стоимость запчастей и расходников рассчитывается отдельно после осмотра.</p>

                    <button id="transfer-to-booking" class="mt-6 w-full py-3.5 px-5 rounded-xl bg-brand-accent hover:bg-brand-accentHover text-white font-semibold text-sm transition-all shadow-lg shadow-orange-500/20">
                        Перенести в заявку на ремонт
                    </button>
                </div>
            </div>
        </div>
    </section>

    // ==================== ПОЧЕМУ ВЫБИРАЮТ НАС ==================== -->
    <section id="features" class="py-16 sm:py-24 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="bg-brand-dark rounded-3xl p-8 sm:p-12 lg:p-16 text-white relative overflow-hidden">
            <div class="absolute bottom-0 right-0 w-80 h-80 bg-brand-accent/10 rounded-full blur-3xl pointer-events-none"/>
            
            <div class="max-w-2xl mb-12">
                <span class="text-brand-accent font-semibold text-sm uppercase tracking-wider">Стандарты качества</span>
                <h2 class="font-display font-extrabold text-3xl sm:text-4xl mt-2">Почему велосипедисты доверяют нам свои байки</h2>
            </div>

            <div class="grid sm:grid-cols-2 lg:grid-cols-4 gap-8">
                <div class="space-y-3">
                    <div class="w-11 h-11 rounded-xl bg-gray-800 border border-gray-700 flex items-center justify-center text-brand-accent font-display font-bold text-lg">01</div>
                    <h3 class="font-display font-bold text-lg">Инструмент Park Tool</h3>
                    <p class="text-sm text-gray-400 leading-relaxed">Используем динамометрические ключи и специализированные съёмники - никаких сорванных граней и перетянутого карбона.</p>
                </div>

                <div class="space-y-3">
                    <div class="w-11 h-11 rounded-xl bg-gray-800 border border-gray-700 flex items-center justify-center text-brand-accent font-display font-bold text-lg">02</div>
                    <h3 class="font-display font-bold text-lg">Запчасти в наличии</h3>
                    <p class="text-sm text-gray-400 leading-relaxed">Держим на складе цепи, колодки, тросы, кассеты и покрышки Shimano, SRAM, KMC и Maxxis - не нужно ждать доставку неделями.</p>
                </div>

                <div class="space-y-3">
                    <div class="w-11 h-11 rounded-xl bg-gray-800 border border-gray-700 flex items-center justify-center text-brand-accent font-display font-bold text-lg">03</div>
                    <h3 class="font-display font-bold text-lg">Фотоотчёт и согласование</h3>
                    <p class="text-sm text-gray-400 leading-relaxed">Если при разборке обнаружится скрытый износ, мы пришлём фото в мессенджер и согласуем стоимость до начала работ.</p>
                </div>

                <div class="space-y-3">
                    <div class="w-11 h-11 rounded-xl bg-gray-800 border border-gray-700 flex items-center justify-center text-brand-accent font-display font-bold text-lg">04</div>
                    <h3 class="font-display font-bold text-lg">Честная гарантия</h3>
                    <p class="text-sm text-gray-400 leading-relaxed">Даём 30 дней гарантии на все выполненные работы. Если тросик вытянется после установки - подстроим бесплатно за 5 минут.</p>
                </div>
            </div>
        </div>
    </section>

    // ==================== ФОРМА ОНЛАЙН-ЗАПИСИ ==================== -->
    <section id="booking" class="py-12 sm:py-20 max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="bg-white rounded-3xl border border-gray-200 shadow-xl p-6 sm:p-10 lg:p-12">
            <div class="text-center max-w-xl mx-auto mb-8">
                <span class="text-brand-accent font-semibold text-sm uppercase tracking-wider">Без очередей</span>
                <h2 class="font-display font-extrabold text-3xl text-gray-900 mt-1">Записаться на ремонт</h2>
                <p class="text-gray-600 text-sm mt-2">Оставьте заявку - мастер свяжется с вами в течение 15 минут, подтвердит время и ответит на вопросы.</p>
            </div>

            <form id="booking-form" class="space-y-5">
                <div class="grid sm:grid-cols-2 gap-5">
                    <div>
                        <label for="client-name" class="block text-sm font-medium text-gray-700 mb-1.5">Ваше имя *</label>
                        <input type="text" id="client-name" required placeholder="Алексей" class="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-brand-accent focus:border-brand-accent outline-none transition-all"/>
                    </div>
                    <div>
                        <label for="client-phone" class="block text-sm font-medium text-gray-700 mb-1.5">Телефон или Telegram *</label>
                        <input type="text" id="client-phone" required placeholder="+7 (999) 123-45-67 или @username" class="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-brand-accent focus:border-brand-accent outline-none transition-all"/>
                    </div>
                </div>

                <div class="grid sm:grid-cols-2 gap-5">
                    <div>
                        <label for="bike-type" class="block text-sm font-medium text-gray-700 mb-1.5">Тип велосипеда</label>
                        <select id="bike-type" class="w-full px-4 py-3 rounded-xl border border-gray-300 bg-white focus:ring-2 focus:ring-brand-accent focus:border-brand-accent outline-none transition-all">
                            <option value="MTB (Горный)">MTB (Горный / Хардтейл / Двухподвес)</option>
                            <option value="Gravel / Шоссе">Гравийный (Gravel) / Шоссейный</option>
                            <option value="Городской / Складной">Городской / Прогулочный / Складной</option>
                            <option value="Электровелосипед">Электровелосипед (E-Bike)</option>
                            <option value="BMX / Детский">BMX / Подростковый / Детский</option>
                        </select>
                    </div>
                    <div>
                        <label for="visit-date" class="block text-sm font-medium text-gray-700 mb-1.5">Желаемая дата визита</label>
                        <input type="date" id="visit-date" class="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-brand-accent focus:border-brand-accent outline-none transition-all"/>
                    </div>
                </div>

                <div>
                    <label for="problem-desc" class="block text-sm font-medium text-gray-700 mb-1.5">Какие работы требуются или что беспокоит?</label>
                    <textarea id="problem-desc" rows="3" placeholder="Например: нужно сезонное ТО, скрипит задний тормоз и плохо переключается на большие звёзды..." class="w-full px-4 py-3 rounded-xl border border-gray-300 focus:ring-2 focus:ring-brand-accent focus:border-brand-accent outline-none transition-all"/>
                </div>

                <button type="submit" class="w-full py-4 px-6 rounded-xl bg-brand-accent hover:bg-brand-accentHover text-white font-semibold text-base transition-all shadow-lg shadow-orange-500/25">
                    Отправить заявку мастеру
                </button>
            </form>
        </div>
    </section>


    }
}
