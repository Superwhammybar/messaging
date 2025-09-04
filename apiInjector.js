async function testApi() {
  const baseUrl = 'http://localhost:3000';
  const queueName = 'test';

  // await create_queues(baseUrl, queueName);
  await add_messages(baseUrl, queueName, 2);
  // await purge_messages(baseUrl, queueName);

  // const messages = await fetch_messages(baseUrl, queueName, 10);
  // console.log(messages);
  // await retry_messages(
  //   baseUrl,
  //   queueName,
  //   messages.map((m) => m.id)
  // );
  // const messages2 = await fetch_messages(baseUrl, queueName, 10);
  // console.log(messages2);

  // const messages3 = await fetch_messages(baseUrl, `${queueName}_dlq`, 10);

  // console.log(messages3);
}

testApi();

async function create_queues(baseUrl, queueName) {
  await fetch(`${baseUrl}/queue`, {
    method: 'POST',
    body: JSON.stringify({ name: `${queueName}_dlq` }),
    headers: { 'Content-Type': 'application/json' },
  });
  await fetch(`${baseUrl}/queue`, {
    method: 'POST',
    body: JSON.stringify({
      name: queueName,
      dlq: { name: `${queueName}_dlq`, delivery_attempts: 1 },
    }),
    headers: { 'Content-Type': 'application/json' },
  });
}

async function add_messages(baseUrl, queueName, count) {
  for (let i = 0; i < count; i++) {
    await fetch(`${baseUrl}/message/add`, {
      method: 'POST',
      body: JSON.stringify({ queue_name: queueName, message: `data${i}` }),
      headers: { 'Content-Type': 'application/json' },
    });
  }
}

async function fetch_messages(baseUrl, queueName, count) {
  const res = await fetch(`${baseUrl}/message/fetch`, {
    method: 'POST',
    body: JSON.stringify({ queue_name: queueName, count }),
    headers: { 'Content-Type': 'application/json' },
  });
  return await res.json();
}

async function remove_messages(baseUrl, queueName, ids) {
  await fetch(`${baseUrl}/message/remove`, {
    method: 'POST',
    body: JSON.stringify({
      queue_name: queueName,
      ids,
    }),
    headers: { 'Content-Type': 'application/json' },
  });
}

async function retry_messages(baseUrl, queueName, ids) {
  await fetch(`${baseUrl}/message/retry`, {
    method: 'POST',
    body: JSON.stringify({
      queue_name: queueName,
      ids,
    }),
    headers: { 'Content-Type': 'application/json' },
  });
}

async function purge_messages(baseUrl, queueName) {
  await fetch(`${baseUrl}/queue/purge`, {
    method: 'POST',
    body: JSON.stringify({
      queue_name: queueName,
    }),
    headers: { 'Content-Type': 'application/json' },
  });
}
