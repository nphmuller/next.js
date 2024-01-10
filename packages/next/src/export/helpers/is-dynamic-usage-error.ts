import { DYNAMIC_ERROR_CODE } from '../../client/components/hooks-server-context'
import { isNavigationSignalError } from './is-navigation-signal-error'

export const isDynamicUsageError = (err: any) =>
  err.digest === DYNAMIC_ERROR_CODE || isNavigationSignalError(err)
