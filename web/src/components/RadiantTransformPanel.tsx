import { useContext, useEffect, useState } from "react";
import { RadiantAppContext } from "../contexts/RadiantAppContext";

export function RadiantTransformPanel() {
    const { response } = useContext(RadiantAppContext);

    const [position, setPosition] = useState({ x: 0, y: 0 });
    const [scale, setScale] = useState({ x: 1, y: 1 });
    const [rotation, setRotation] = useState(0);

    useEffect(() => {
        if (response?.NodeSelected) {
            let node = response.NodeSelected.Rectangle;
            let transform = node.transform;
            setPosition({ x: transform.position[0], y: transform.position[1]});
            setScale({ x: transform.scale[0], y: transform.scale[1]});
            setRotation(transform.rotation);
        }
    }, [response]);

    return (
        <div>
            <h1>Transform</h1>
            <div>
                <label>Position</label>
                <input type="number" value={position.x} onChange={(e) => {
                    setPosition({ x: parseFloat(e.target.value), y: position.y });
                }} />
                <input type="number" value={position.y} onChange={(e) => {
                    setPosition({ x: position.x, y: parseFloat(e.target.value) });
                }} />
            </div>
            <div>
                <label>Scale</label>
                <input type="number" value={scale.x} onChange={(e) => {
                    setScale({ x: parseFloat(e.target.value), y: scale.y });
                }} />
                <input type="number" value={scale.y} onChange={(e) => {
                    setScale({ x: scale.x, y: parseFloat(e.target.value) });
                }} />
            </div>
            <div>
                <label>Rotation</label>
                <input type="number" value={rotation} onChange={(e) => {
                    setRotation(parseFloat(e.target.value));
                }} />
            </div>
        </div>
    );
}
