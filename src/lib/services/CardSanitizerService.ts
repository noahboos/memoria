export class CardSanitizerService {
    static ALLOWED_TAGS: Set<string> = new Set([
        "div",
        "p"
    ]);

    static sanitize(markup: string) {
        let parser: DOMParser = new DOMParser();
        let document: Document = parser.parseFromString(markup, "text/html");

        this.sanitizeNode(document.body);

        return document.body.innerHTML;
    }

    static sanitizeNode(node: Node) {
        if (node.nodeType === Node.ELEMENT_NODE) {
            const element = node as HTMLElement;
            const tagName: string = element.tagName.toLowerCase();

            if (tagName !== "body" && tagName !== "html") {
                if (!this.ALLOWED_TAGS.has(tagName)) {
                    element.replaceWith(...Array.from(element.childNodes));
                }

                for (const attribute of element.attributes) {
                    element.removeAttribute(attribute.name);
                }
            }
        }

        for (const child of Array.from(node.childNodes)) {
            this.sanitizeNode(child);
        }
    }
}