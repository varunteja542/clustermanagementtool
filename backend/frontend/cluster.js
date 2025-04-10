// Fetch clusters on page load.
fetch("/clusters")
  .then(res => res.json())
  .then(clusters => {
    const div = document.getElementById("clusters");
    div.innerHTML = "<h2>Select a Cluster:</h2>";
    clusters.forEach(c => {
      const btn = document.createElement("button");
      btn.innerText = c.name;
      btn.onclick = () => loadNodes(c.id);
      div.appendChild(btn);
    });
  });

function loadNodes(clusterId) {
  fetch(`/clusters/${clusterId}/nodes`)
    .then(res => res.json())
    .then(nodes => {
      const nodeDiv = document.getElementById("nodes");
      nodeDiv.innerHTML = `<h2>Nodes in Cluster ${clusterId}</h2>`;
      nodes.forEach(n => {
        const el = document.createElement("div");
        el.innerHTML = `
          <b>Node:</b> ${n.id} — <b>Status:</b> ${n.status}
          <button onclick="viewActions('${n.id}')">View Actions</button>
          <button onclick="deleteNode(${clusterId}, '${n.id}')">Delete</button>
        `;
        nodeDiv.appendChild(el);
      });
    });
}

function deleteNode(clusterId, nodeId) {
  fetch(`/clusters/${clusterId}/nodes/${nodeId}`, { method: "DELETE" })
    .then(res => res.json())
    .then(msg => alert(msg.message));
}

function viewActions(nodeId) {
  fetch(`/nodes/${nodeId}/actions`)
    .then(res => res.json())
    .then(actions => {
      alert(`Actions for ${nodeId}: ${JSON.stringify(actions)}`);
    });
}
