interface PudeukoObject {
  items: PudeukoItem[],
}

interface PudeukoLink {
  url: string,
}

interface PudeukoItem {
  id: string,
  createdAt: Date,
  link?: PudeukoLink,
  text: string,
}

export {
  PudeukoObject,
  PudeukoItem,
  PudeukoLink,
};
