


function arrayBufferToBase64( buffer ) {
    var binary = '';
    var bytes = new Uint8Array( buffer );
    var len = bytes.byteLength;
    for (var i = 0; i < len; i++) {
        binary += String.fromCharCode( bytes[ i ] );
    }
    return window.btoa( binary );
}



function base64ToArrayBuffer(base64) {
    var binary_string =  window.atob(base64);
    var len = binary_string.length;
    var bytes = new Uint8Array( len );
    for (var i = 0; i < len; i++)        {
        bytes[i] = binary_string.charCodeAt(i);
    }
    return bytes.buffer;
}

let a = r
a.arrayBuffer().then(d => {

    let encoded = arrayBufferToBase64(d);

    var decoded = base64ToArrayBuffer(encoded)

    // var encoded=base64ArrayBuffer(d);
    // var decoded=decode_ascii85(encoded);

    console.log(d)
    console.log(decoded)

    // if (decoded === d) {
    //     console.log("exact")
    // }
    // console.log(encoded)
})

