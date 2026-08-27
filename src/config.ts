import raw from "../config/mail.json";

export interface AddressBookEntry {
  address: string;
  note?: string;
}

export interface MailConfig {
  host: string;
  app: {
    title: string;
    subtitle: string;
    loginTagline: string;
    loginHeadlineBefore: string;
    loginHeadlineEm: string;
    loginLead: string;
    loginPoints: string[];
    loginFormTitle: string;
    loginFormSub: string;
  };
  mail: {
    fromEmail: string;
    fromNameDefault: string;
    contactEmail: string;
    brevoTag: string;
  };
  site: {
    url: string;
    label: string;
    brandName: string;
    logoPath: string;
  };
  brand: {
    tile: string;
    tileEdge: string;
    accent: string;
    cream: string;
    siteBlue: string;
  };
  addressBook: AddressBookEntry[];
}

function isRecord(v: unknown): v is Record<string, unknown> {
  return v !== null && typeof v === "object" && !Array.isArray(v);
}

function reqString(obj: Record<string, unknown>, key: string): string {
  const v = obj[key];
  if (typeof v !== "string" || !v.trim()) {
    throw new Error(`config/mail.json: missing or invalid "${key}"`);
  }
  return v.trim();
}

function parseAddressBook(rawBook: unknown): AddressBookEntry[] {
  if (!Array.isArray(rawBook)) return [];
  const out: AddressBookEntry[] = [];
  for (const item of rawBook) {
    if (!isRecord(item)) continue;
    const address = reqString(item, "address").toLowerCase();
    if (!address.includes("@")) continue;
    const noteRaw = item.note;
    const note = typeof noteRaw === "string" && noteRaw.trim() ? noteRaw.trim() : undefined;
    out.push(note ? { address, note } : { address });
  }
  return out;
}

function validateMailConfig(input: unknown): MailConfig {
  if (!isRecord(input)) throw new Error("config/mail.json: root must be an object");
  const app = input.app;
  const mail = input.mail;
  const site = input.site;
  const brand = input.brand;
  if (!isRecord(app) || !isRecord(mail) || !isRecord(site) || !isRecord(brand)) {
    throw new Error("config/mail.json: app, mail, site, brand are required");
  }
  const loginPoints = app.loginPoints;
  if (!Array.isArray(loginPoints) || loginPoints.some((p) => typeof p !== "string")) {
    throw new Error("config/mail.json: app.loginPoints must be string[]");
  }
  return {
    host: reqString(input, "host"),
    app: {
      title: reqString(app, "title"),
      subtitle: reqString(app, "subtitle"),
      loginTagline: reqString(app, "loginTagline"),
      loginHeadlineBefore: reqString(app, "loginHeadlineBefore"),
      loginHeadlineEm: reqString(app, "loginHeadlineEm"),
      loginLead: reqString(app, "loginLead"),
      loginPoints: loginPoints.map((p) => String(p).trim()).filter(Boolean),
      loginFormTitle: reqString(app, "loginFormTitle"),
      loginFormSub: reqString(app, "loginFormSub"),
    },
    mail: {
      fromEmail: reqString(mail, "fromEmail").toLowerCase(),
      fromNameDefault: reqString(mail, "fromNameDefault"),
      contactEmail: reqString(mail, "contactEmail").toLowerCase(),
      brevoTag: reqString(mail, "brevoTag"),
    },
    site: {
      url: reqString(site, "url").replace(/\/$/, ""),
      label: reqString(site, "label"),
      brandName: reqString(site, "brandName"),
      logoPath: reqString(site, "logoPath").startsWith("/")
        ? reqString(site, "logoPath")
        : `/${reqString(site, "logoPath")}`,
    },
    brand: {
      tile: reqString(brand, "tile"),
      tileEdge: reqString(brand, "tileEdge"),
      accent: reqString(brand, "accent"),
      cream: reqString(brand, "cream"),
      siteBlue: reqString(brand, "siteBlue"),
    },
    addressBook: parseAddressBook(input.addressBook),
  };
}

export const mailConfig: MailConfig = validateMailConfig(raw);

export function mailOrigin(): string {
  return `https://${mailConfig.host}`;
}

export function mailLogoUrl(): string {
  return `${mailOrigin()}${mailConfig.site.logoPath}`;
}

export function siteLogoCanonicalUrl(): string {
  const base = mailConfig.site.url.replace(/\/$/, "");
  const path = mailConfig.site.logoPath.startsWith("/")
    ? mailConfig.site.logoPath
    : `/${mailConfig.site.logoPath}`;
  return `${base}${path}`;
}
