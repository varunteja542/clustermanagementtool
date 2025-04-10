// Fetch clusters from backend and display as buttons.
fetch("/clusters")
  .then(res => res.json())
  .then(data => {
    const div = document.getElementById("clusters");
    div.innerHTML = "<h2>Select a Cluster:</h2>";
    data.clusters.forEach(c => {
      const btn = document.createElement("button");
      btn.innerText = c.name;
      btn.onclick = () => loadNodes(c.id);
      div.appendChild(btn);
    });
  });

function loadNodes(clusterId) {
  fetch(`/clusters/${clusterId}/nodes`)
    .then(res => res.json())
    .then(data => {
      const nodeDiv = document.getElementById("nodes");
      nodeDiv.innerHTML = `<h2>Nodes in Cluster ${clusterId}</h2>`;
      data.nodes.forEach(n => {
        const el = document.createElement("div");
        el.className = "node";
        el.innerHTML = `
          <strong>Node:</strong> ${n.id} — <strong>Status:</strong> ${n.status}
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
    .then(data => {
      alert(`Actions for ${nodeId}: ${JSON.stringify(data.actions)}`);
    });
}
