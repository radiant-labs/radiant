import { RadiantAppController } from "radiant-runtime";
export class RadiantController {
    constructor(controller) {
        this._controller = controller;
    }
    static async createController(f) {
        return new RadiantController(await new RadiantAppController(f));
    }
    /**
     * Activates the provided tool.
     *
     * @param tool the tool to activate.
     */
    activateTool(toolId) {
        this._controller.handleMessage({
            SceneMessage: {
                SelectTool: {
                    id: toolId,
                },
            },
        });
    }
    addRectangle(position, scale) {
        this._controller.handleMessage({
            AddRectangle: {
                position,
                scale,
            },
        });
    }
    addImage(path, name = "", position = [100, 100], scale = [100, 100]) {
        this._controller.handleMessage({
            AddImage: {
                name,
                path,
            },
        });
    }
    setTransform(nodeId, position, scale) {
        this._controller.handleMessage({
            SceneMessage: {
                SetTransform: {
                    id: nodeId,
                    position,
                    scale,
                },
            },
        });
    }
    setFillColor(nodeId, color) {
        this._controller.handleMessage({
            SceneMessage: {
                SetFillColor: {
                    id: nodeId,
                    fill_color: color,
                },
            },
        });
    }
    setStrokeColor(nodeId, color) {
        this._controller.handleMessage({
            SceneMessage: {
                SetStrokeColor: {
                    id: nodeId,
                    stroke_color: color,
                },
            },
        });
    }
}
//# sourceMappingURL=index.js.map