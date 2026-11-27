use leptos::prelude::*;

#[component]
pub fn SuccessModal() -> impl IntoView {
    view! {
     <div id="success-modal" class="fixed inset-0 z-50 hidden items-center justify-center bg-black/70 backdrop-blur-sm p-4">
        <div class="bg-white rounded-3xl max-w-md w-full p-7 text-center shadow-2xl transform transition-all">
            <div class="w-16 h-16 rounded-full bg-emerald-100 text-emerald-600 flex items-center justify-center mx-auto mb-4">
                <svg class="w-8 h-8" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
                </svg>
            </div>
            <h3 class="font-display font-extrabold text-2xl text-gray-900">Заявка принята!</h3>
            <p id="modal-summary" class="text-sm text-gray-600 mt-2 leading-relaxed">
                Спасибо! Мастер уже получил вашу заявку и свяжется с вами в ближайшее время.
            </p>
            <button id="close-modal-btn" class="mt-6 w-full py-3.5 px-5 rounded-xl bg-brand-dark hover:bg-gray-800 text-white font-semibold text-sm transition-colors">
                Отлично, жду звонка
            </button>
        </div>
    </div>

    }
}
