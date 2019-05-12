codes = new WebSocket('ws://' + document.domain + ':' + location.port + '/lobby_ws');
s.onmessage = function(msg) {
  console.log('Got server response:');
  console.log(msg);
  try {
    d = JSON.parse(msg.data);
  } catch (e) {
    console.log(e);
    return;
  }
  if (!('command' in d)) {
    console.log('No command in response from server. ' + msg);
    return;
  }

  switch (d['command']) {
    case 'join_game': {
      window.location.href = d['game']
    }
  }
}

function join_or_create() {
  var name = document.getElementById('name').value;
  var room = document.getElementById('room').value;
  if (!name) {
    iziToast.error({
      title: 'Error',
      message: 'Name cannot be empty'
    });
    return;
  }
  if (!room) {
    iziToast.error({
      title: 'Error',
      message: 'Room cannot be empty'
    });
    return;
  }
  s.send(JSON.stringify({ 'command': 'join_or_create_game', 'name': name, 'room': room }));
}
