type TNode = {
  name: string;
  index: number;
};

const n1: TNode = {
  name: "Node 1",
  index: 1,
};

function changeNodeName(node: TNode) {
  node.name = node.name + "NEW";
}

changeNodeName(n1);
console.log("Name of node n1", n1.name);

/// data races

n1.name = "Brand new name";
console.log("Name of node n1", n1.name);
