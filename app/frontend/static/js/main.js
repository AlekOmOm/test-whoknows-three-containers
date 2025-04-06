document.addEventListener('DOMContentLoaded', () => {
    const searchForm = document.getElementById('searchForm');
    const searchInput = document.getElementById('searchInput');

    // Focus the search input
    searchInput.focus();

    // Optional: Add loading state when form is submitted
    searchForm.addEventListener('submit', (e) => {
        const button = searchForm.querySelector('button');
        button.disabled = true;
        button.textContent = 'Searching...';
    });
});