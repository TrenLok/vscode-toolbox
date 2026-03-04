/* eslint-disable ts/no-explicit-any */
interface StoreOptions<T> {
  file: string;
  key: string;
  defaultValue: T;
  logPrefix: string;
  latestVersion: number;
  migrations?: Record<number, (data: any) => any>;
}

interface VersionedStore<T> {
  version: number;
  data: T;
}

export function useTauriStore<T>({
  file,
  key,
  defaultValue,
  logPrefix,
  latestVersion,
  migrations = {},
}: StoreOptions<T>) {
  async function save(value: T) {
    try {
      const store = await useTauriStoreLoad(file);
      const payload: VersionedStore<T> = {
        version: latestVersion,
        data: value,
      };
      await store.set(key, payload);
      await store.save();
    } catch (error_) {
      useTauriLogError(`${logPrefix} save error: ${error_}`);
    }
  }

  async function load(): Promise<T> {
    try {
      const store = await useTauriStoreLoad(file);
      const has = await store.has(key);

      if (!has) {
        const initial: VersionedStore<T> = {
          version: latestVersion,
          data: defaultValue,
        };
        await store.set(key, initial);
        await store.save();
        return defaultValue;
      }

      const raw
        = (await store.get<VersionedStore<any>>(key))
          ?? ({ version: 1, data: defaultValue } as VersionedStore<any>);

      let currentVersion = raw.version ?? 1;
      let data = raw.data;

      while (currentVersion < latestVersion) {
        const migrate = migrations[currentVersion];
        if (!migrate) break;
        data = migrate(data);
        currentVersion += 1;
      }

      const migrated: VersionedStore<T> = {
        version: latestVersion,
        data,
      };

      await store.set(key, migrated);
      await store.save();

      return migrated.data;
    } catch (error_) {
      useTauriLogError(`${logPrefix} load error: ${error_}`);
      return defaultValue;
    }
  }

  async function clear() {
    try {
      const store = await useTauriStoreLoad(file);
      const payload: VersionedStore<T> = {
        version: latestVersion,
        data: defaultValue,
      };
      await store.set(key, payload);
      await store.save();
    } catch (error_) {
      useTauriLogError(`${logPrefix} clear error: ${error_}`);
    }
  }

  return {
    save,
    load,
    clear,
  };
}
