$('button.add-row').click((e) => {
  const parent = e.currentTarget.parentElement.parentElement
  const clone = $(parent).clone(true, true).appendTo(parent.parentElement)
  $(clone).find("input").val(null)
})

$('td.content-cell').focusout((e) => {
  console.log($(e.currentTarget).find('input').val())
})
