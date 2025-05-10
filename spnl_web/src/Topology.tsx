import { match, P } from "ts-pattern"
import { useMemo, useState } from "react"
import { Tree, Data } from "react-tree-graph"

import "./Topology.css"

type Props = {
  unit: null | import("./Unit").Unit
}

const NODE_SIZE = 32
const LABEL_FONT_SIZE = 16

function node(id: string, label: string, children: Data[] = []): Data {
  return {
    label,
    name: id + "." + label,
    labelProp: "label",
    children,
    nodeProps: { width: NODE_SIZE, height: NODE_SIZE },
    gProps: {
      className: "node spnl-node spnl-node-" + (label === "+" ? "plus" : label),
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
        margins={{ bottom: 0, left: NODE_SIZE, top: 0, right: NODE_SIZE }}
        height={600}
        width={400}
        nodeShape="rect"
        textProps={{
          dx: -(NODE_SIZE / 2 - LABEL_FONT_SIZE / 2),
          dy: NODE_SIZE / 2 - LABEL_FONT_SIZE / 2,
        }}
        svgProps={{
          transform: "rotate(90)", //rotates the tree to make it verticle
          viewBox: "0 200 600 400",
        }}
        textProps={{
          transform: "rotate(-90)", //rotates the text label
          x: -NODE_SIZE * 0.75,
          y: 2,
        }}
      />
    )
  }
}
