interface PudeukoObject {
  items: PudeukoItem[],
  archive: PudeukoItem[],
}

interface PudeukoLink {
  url: string,
}

interface PudeukoIcon {
  src: string,
}

interface PudeukoItem {
  id: string,
  text: string,
  link?: PudeukoLink,
  icon?: PudeukoIcon,
  createdAt: Date,
}

export {
  PudeukoObject,
  PudeukoItem,
  PudeukoLink,
};
