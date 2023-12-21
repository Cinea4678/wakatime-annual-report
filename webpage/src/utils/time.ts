/**
 * 将秒数的时间格式化
 * @param sec 时间
 * @param with_space 是否需要在汉字与数字间加入空格
 * @param with_seconds 是否需要秒
 */
export function FormatSeconds(
  sec: number,
  with_space?: boolean,
  with_seconds?: boolean,
) {
  const sp = with_space ? " " : ""

  let result = ""

  if (sec > 3600) {
    result += `${Math.floor(sec / 3600)}${sp}小时${sp}`
  }
  if (sec > 60) {
    result += `${Math.floor((sec % 3600) / 60)}${sp}分${sp}`
  }
  if (sec > 0 && with_seconds !== false) {
    result += `${Math.floor(sec % 60)}${sp}秒`
  }
  return result
}

/**
 * 将秒数的时间格式化并携带样式
 * @param sec 时间
 * @param with_space 是否需要在汉字与数字间加入空格
 * @param with_seconds 是否需要秒
 */
export function FormatSecondsHtml(
  sec: number,
  with_space?: boolean,
  with_seconds?: boolean,
) {
  const sp = with_space ? " " : ""

  let result = ""

  if (sec > 3600) {
    result += `<span class="dig">${Math.floor(sec / 3600)}</span>${sp}小时${sp}`
  }
  if (sec > 60) {
    result += `<span class="dig">${Math.floor(
      (sec % 3600) / 60,
    )}</span>${sp}分${sp}`
  }
  if (sec > 0 && with_seconds !== false) {
    result += `<span class="dig">${Math.floor(sec % 60)}</span>${sp}秒`
  }
  return result
}
