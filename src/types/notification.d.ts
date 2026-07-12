export {}
declare global {
  namespace Notify {
    type Subscriber = (info: Info) => void
    type Type = 'info' | 'success' | 'warning' | 'error'
    type Info = { id: number; type: Type; message: string; duration?: number }
    type Options = { message: string; duration?: number }
    type Method = { (content: string, type: Type): void; (options: Options, type: Type): void }

    interface State {
      counter: number
      subscribers: Subscriber[]
      subscribe: (subscriber: Subscriber) => void
      publish: (info: Info) => void
      create: (type: Type) => (info: string | Info) => void
    }
  }
}
