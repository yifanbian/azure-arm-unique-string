import { unique_string } from '../wasm-impl/pkg/';

function main() {
    var container = document.getElementById('container');
    var result = document.getElementById('result');
    document.getElementById('btnAdd').onclick = () => container?.appendChild(createNewRow());
    document.getElementById('btnCompute').onclick = () => compute_hash();

    function createNewRow() {
        var row = document.createElement('div');
        var input = document.createElement('input');
        var btnRemove = document.createElement('button');
        btnRemove.innerText = 'Remove this parameter';
        btnRemove.onclick = (ev) => ev.target && ev.target.parentElement.remove();
        row.appendChild(input);
        row.append(btnRemove);
        return row;
    }

    function compute_hash() {
        var val = [...document.querySelectorAll('#container > div > input')].map(n => n.value).join('-');
        var hash = unique_string(val);
        result.value = hash;
    }
}

main();