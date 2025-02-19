export enum ActivityType {
  Playing = 0,
  Listening = 2,
  Watching = 3,
  Competing = 5,
};

export class Timestamps {
  protected start: number;
  protected end?: number;

  constructor(start: number, end?: number) {
    this.start = start;
    this.end = end;
  }
}

export class Party {
  protected id: string;
  protected size: [number, 2];

  constructor(id: string, size: [number, 2]) {
    this.id = id;
    this.size = size;
  }
}

export class Assets {
  protected large_image?: string;
  protected large_text?: string;
  protected small_image?: string;
  protected small_text?: string;

  setLargeImage(x: string): this {
    this.large_image = x;
    return this;
  }

  setLargeText(x: string): this {
    this.large_text = x;
    return this;
  }

  setSmallImage(x: string): this {
    this.small_image = x;
    return this;
  }

  setSmallText(x: string): this {
    this.small_text = x;
    return this;
  }
}

export class Secrets {
  protected join?: string;
  protected spectate?: string;
  protected match?: string;

  setJoin(join: string): this {
    this.join = join;
    return this;
  }

  setSpectate(spectate: string): this {
    this.spectate = spectate;
    return this;
  }

  setMatch(match: string): this {
    this.match = match;
    return this;
  }
}

export class Button {
  protected label: string;
  protected url: string;

  constructor(label: string, url: string) {
    this.label = label;
    this.url = url;
  }
}

export class Activity {
  protected state?: string;
  protected details?: string;
  protected timestamps?: Timestamps;
  protected party?: Party;
  protected assets?: Assets;
  protected secrets?: Secrets;
  protected buttons?: Button[];
  protected activity_type?: ActivityType;

  setState(state: string): this {
    this.state = state;
    return this;
  }

  setDetails(details: string): this {
    this.details = details;
    return this;
  }

  setTimestamps(timestamps: Timestamps): this {
    this.timestamps = timestamps;
    return this;
  }

  setParty(party: Party): this {
    this.party = party;
    return this;
  }

  setAssets(assets: Assets): this {
    this.assets = assets;
    return this;
  }

  setSecrets(secrets: Secrets): this {
    this.secrets = secrets;
    return this;
  }

  setButton(buttons: Button[]): this {
    this.buttons = buttons;
    return this;
  }

  setActivity(activity: ActivityType): this {
    this.activity_type = activity;
    return this;
  }

  toString(): string {
    return JSON.stringify(this);
  }
}