let expenses = {};

function addExpense() {
    const category = document.getElementById('category').value;
    const amount = parseFloat(document.getElementById('amount').value);

    if (!category || isNaN(amount)) {
        alert('Please enter valid category and amount.');
        return;
    }

    if (!expenses[category]) {
        expenses[category] = 0;
    }
    expenses[category] += amount;

    document.getElementById('category').value = '';
    document.getElementById('amount').value = '';
}

function viewReport() {
    let report = 'Expenses Report:\n';
    for (const [category, amount] of Object.entries(expenses)) {
        report += `${category}: ${amount.toFixed(2)}\n`;
    }
    document.getElementById('report').textContent = report;
}

function exportToCSV() {
    let csvContent = 'Category,Amount\n';
    for (const [category, amount] of Object.entries(expenses)) {
        csvContent += `${category},${amount.toFixed(2)}\n`;
    }

    const blob = new Blob([csvContent], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.setAttribute('href', url);
    a.setAttribute('download', 'expenses.csv');
    a.click();
}

document.querySelector('a.btn').addEventListener('click', function(event) {
    event.preventDefault();
    window.open('https://github.com/Rambo9836/expense', '_blank');
});
