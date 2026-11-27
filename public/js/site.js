document.addEventListener('DOMContentLoaded', () => {
    // 4. Установка сегодняшней даты по умолчанию в форме записи
    const visitDateInput = document.getElementById('visit-date');
    if (visitDateInput) {
        const today = new Date().toISOString().split('T')[0];
        visitDateInput.min = today;
        if (!visitDateInput.value) {
            visitDateInput.value = today;
        }
    }

    let selectedServices = [];
    let currentTotal = 0;

    function updateCalculator() {
        const checkboxes = document.querySelectorAll('.service-checkbox');
        const selectedListEl = document.getElementById('selected-list');
        const totalPriceEl = document.getElementById('total-price');
        const clearCalcBtn = document.getElementById('clear-calc-btn');

        selectedServices = [];
        currentTotal = 0;

        checkboxes.forEach((cb) => {
            if (cb.checked) {
                const name = cb.getAttribute('data-name') || '';
                const price = parseInt(cb.getAttribute('data-price') || '0', 10) || 0;
                selectedServices.push({ name, price });
                currentTotal += price;
            }
        });

        if (selectedListEl) {
            if (selectedServices.length === 0) {
                selectedListEl.innerHTML =
                    '<p class="text-gray-500 italic">Выберите услуги в списке слева, чтобы рассчитать итоговую сумму...</p>';
                if (clearCalcBtn) clearCalcBtn.classList.add('hidden');
            } else {
                if (clearCalcBtn) clearCalcBtn.classList.remove('hidden');
                selectedListEl.innerHTML = selectedServices
                    .map(
                        (item) => `
                        <div class="flex justify-between items-start gap-2">
                            <span class="text-gray-200">${item.name}</span>
                            <span class="font-semibold text-white whitespace-nowrap">${item.price.toLocaleString('ru-RU')} ₽</span>
                        </div>
                    `
                    )
                    .join('');
            }
        }

        if (totalPriceEl) {
            totalPriceEl.textContent = `${currentTotal.toLocaleString('ru-RU')} ₽`;
        }
    }

    // Изменения чекбоксов калькулятора
    document.addEventListener('change', (event) => {
        if (event.target.closest('.service-checkbox')) {
            updateCalculator();
        }
    });

    // Все клики через делегирование
    document.addEventListener('click', (event) => {
        const target = event.target;

        // 1. Мобильное меню
        if (target.closest('#mobile-menu-btn')) {
            const mobileMenu = document.getElementById('mobile-menu');
            if (mobileMenu) mobileMenu.classList.toggle('hidden');
            return;
        }

        if (target.closest('.mobile-link')) {
            const mobileMenu = document.getElementById('mobile-menu');
            if (mobileMenu) mobileMenu.classList.add('hidden');
            return;
        }

        // 2. Фильтрация прайс-листа
        const filterBtn = target.closest('.filter-btn');
        if (filterBtn) {
            const filterBtns = document.querySelectorAll('.filter-btn');
            const serviceRows = document.querySelectorAll('.service-row');

            filterBtns.forEach((b) => {
                b.classList.remove('bg-brand-dark', 'text-white');
                b.classList.add('bg-white', 'text-gray-700');
            });
            filterBtn.classList.remove('bg-white', 'text-gray-700');
            filterBtn.classList.add('bg-brand-dark', 'text-white');

            const filter = filterBtn.getAttribute('data-filter');
            serviceRows.forEach((row) => {
                if (filter === 'all' || row.getAttribute('data-category') === filter) {
                    row.classList.remove('hidden');
                } else {
                    row.classList.add('hidden');
                }
            });
            return;
        }

        // 3a. Очистка калькулятора
        if (target.closest('#clear-calc-btn')) {
            document.querySelectorAll('.service-checkbox').forEach((cb) => {
                cb.checked = false;
            });
            updateCalculator();
            return;
        }

        // 3b. Перенос выбранных услуг в форму записи
        if (target.closest('#transfer-to-booking')) {
            const problemDescInput = document.getElementById('problem-desc');
            if (problemDescInput && selectedServices.length > 0) {
                const summaryText = selectedServices
                    .map((s) => `• ${s.name} (${s.price} ₽)`)
                    .join('\n');
                problemDescInput.value = `Выбранные услуги из калькулятора:\n${summaryText}\nИтого ориентировочно: ${currentTotal} ₽`;
            }
            const bookingEl = document.getElementById('booking');
            if (bookingEl) bookingEl.scrollIntoView({ behavior: 'smooth' });
            return;
        }

        // 5a. Закрытие модального окна по кнопке
        const successModal = document.getElementById('success-modal');
        if (target.closest('#close-modal-btn')) {
            if (successModal) {
                successModal.classList.add('hidden');
                successModal.classList.remove('flex');
            }
            return;
        }

        // 5b. Закрытие модального окна по клику на фон
        if (successModal && target === successModal) {
            successModal.classList.add('hidden');
            successModal.classList.remove('flex');
        }
    });

    // 5c. Отправка формы записи
    document.addEventListener('submit', (event) => {
        const bookingForm = event.target.closest('#booking-form');
        if (!bookingForm) return;

        event.preventDefault();
        const name = document.getElementById('client-name')?.value.trim() || '';
        const phone = document.getElementById('client-phone')?.value.trim() || '';
        const bike = document.getElementById('bike-type')?.value || '';

        const modalSummary = document.getElementById('modal-summary');
        const successModal = document.getElementById('success-modal');

        if (modalSummary) {
            modalSummary.textContent = `${name}, ваша заявка на обслуживание велосипеда (${bike}) принята. Мастер свяжется с вами по контакту ${phone} в течение 15 минут.`;
        }
        if (successModal) {
            successModal.classList.remove('hidden');
            successModal.classList.add('flex');
        }
        bookingForm.reset();
    });
});
