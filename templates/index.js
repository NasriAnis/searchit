async function doSearch(query) {
    const res = await fetch('/search', { method: 'POST', body: query });
    const documents = await res.json(); // [{ path, extension, page, score }, ...]

    const container = document.getElementById('results');
    container.innerHTML = '';

    documents.forEach(doc => {
        const li = document.createElement('li');
        const a = document.createElement('a');
        // encodeURIComponent turns "/" into "%2F" — matches the urlencoding decode server-side
        if (doc.extension == "pdf")
          a.href = `/files/${encodeURIComponent(doc.path)}#page=${doc.page}`;
        else if (doc.extension == "html")
          a.href = `/files/${encodeURIComponent(doc.path)}`;
        a.target = '_blank';
        a.textContent = `${doc.path} (page ${doc.page}) (File type ${doc.extension})`;
        li.appendChild(a);
        container.appendChild(li);
    });
}

function sendSearch() {
    const query = document.getElementById('search').value;
    doSearch(query);
}