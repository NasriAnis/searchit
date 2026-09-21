async function doSearch(query) {
    const res = await fetch('/search', { method: 'POST', body: query });
    const documents = await res.json(); // [{ path, extension, page, score }, ...]

    const container = document.getElementById('results');
    container.innerHTML = '';

    documents.forEach(doc => {
        const li = document.createElement('li');
        const a = document.createElement('a');
        // encodeURIComponent turns "/" into "%2F" — matches the urlencoding decode server-side
        a.href = `/files/${encodeURIComponent(doc.path)}#page=${doc.page}`;
        a.target = '_blank';
        a.textContent = `${doc.path} (page ${doc.page})`;
        li.appendChild(a);
        container.appendChild(li);
    });
}

function sendSearch() {
    const query = document.getElementById('search').value;
    doSearch(query);
}