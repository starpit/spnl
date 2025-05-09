import { match, P } from "ts-pattern"
import { useMemo, useState } from "react"
import { Tree, Data } from "react-tree-graph"

import "./Topology.css"

type Props = {
  unit: null | import("./Unit").Unit
}

function node(id: string, label: string, children: Data[] = []): Data {
  return {
    name: label,
    keyProp: id + "." + label,
    children,
    nodeProps: { width: 20, height: 20 },
    gProps: {
      class: "node spnl-node spnl-node-" + (label === "+" ? "plus" : label),
    },
  }
}

function graphify(unit: import("./Unit").Unit, id = "root"): [Data] {
  return match(unit)
    .with({ user: P.array(P.string) }, () => [node(id, "U")])
    .with({ system: P.array(P.string) }, () => [node(id, "S")])
    .with({ g: P.array() }, ({ g }) => [
      node(id, "G", graphify(g[1], id + ".G")),
    ])
    .with({ print: P._ }, () => [])
    .with({ repeat: P.array() }, ({ repeat }) =>
      Array(repeat[0])
        .fill(0)
        .flatMap((_, idx) => graphify(repeat[1], id + "." + idx)),
    )
    .with({ cross: P.array() }, ({ cross }) => [
      node(
        id,
        "X",
        cross.flatMap((child) => graphify(child, id + ".X")),
      ),
    ])
    .with({ plus: P.array() }, ({ plus }) => [
      node(
        id,
        "+",
        plus.flatMap((child) => graphify(child, id + ".+")),
      ),
    ])
    .exhaustive()

  return { nodes: NODES, edges: EDGES }
}

export default function Topology(props: Props) {
  const data = useMemo(
    () => (!props.unit ? null : graphify(props.unit)[0]),
    [props.unit],
  )
  if (!data) {
    return <></>
  } else {
    return (
      <Tree
        key={JSON.stringify(data)}
        data={data}
        height={400}
        width={400}
        nodeShape="rect"
        textProps={{
          x: "-14",
          y: "-1",
        }}
      />
    )
  }
}
