class ValidExplicit {
  public constructor(public foo: string) {}
}

class ValidExplicit2 {
  public constructor(private readonly foo: string) {}
}

class ValidExplicit3 {
  public constructor(protected foo: string) {}
}

class ValidExplicit4 {
  protected name: string;
  private x: number;
  public getX() {
    return this.x;
  }
}

class ValidExplicit5 {
  protected name: string;
  protected foo?: string;
  public 'foo-bar'?: string;
}

class ValidExplicit6 {
  public constructor({ x, y }: { x: number; y: number }) {}
}

class ValidExplicit7 {
  protected name: string;
  protected x?: string;
  public getX() {
    return this.x;
  }
}

class ValidExplicit8 {
  private x = 2;
}

class ValidExplicit9 {
  #foo = 1;
  #bar() {}
}
