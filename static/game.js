s = new WebSocket('ws://' + document.domain + ':' + location.port + '/game_ws');

s.onerror = function(e) {
    console.log("ERROR:");
    console.log(e);
}

s.onmessage = function(msg) {
  console.log('Got server response:');
  console.log(msg);
  if (!msg.data) return;
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

function join_a() {
  s.send(JSON.stringify({ 'command': 'join_a' }));
}

function join_b() {
  s.send(JSON.stringify({ 'command': 'join_b' }));
}

function leave_team() {
  s.send(JSON.stringify({ 'command': 'leave_team' }));
}
