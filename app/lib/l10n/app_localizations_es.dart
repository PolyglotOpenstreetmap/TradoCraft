// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Spanish Castilian (`es`).
class AppLocalizationsEs extends AppLocalizations {
  AppLocalizationsEs([String locale = 'es']) : super(locale);

  @override
  String get accepted => 'Aceptado';

  @override
  String get accessToken => 'Token de acceso';

  @override
  String get account => 'Cuenta';

  @override
  String get action => 'Acción';

  @override
  String get actionAddTag => 'Añadir etiqueta';

  @override
  String get actionSetCategory => 'Asignar categoría';

  @override
  String get actionSetInternal => 'Marcar como interno';

  @override
  String get actionSetOwner => 'Asignar responsable';

  @override
  String get actionSetPriority => 'Asignar prioridad';

  @override
  String get actionSetTeam => 'Asignar equipo';

  @override
  String get actions => 'Acciones';

  @override
  String get activate => 'Activar';

  @override
  String get active => 'Activo';

  @override
  String activeSince(String date) {
    return 'Activa desde el $date. Los cambios se hacen con una nueva revisión.';
  }

  @override
  String get add => 'Añadir';

  @override
  String get addAction => 'Añadir acción';

  @override
  String get addClosure => 'Añadir cierre';

  @override
  String get addComponent => 'Añadir componente';

  @override
  String get addComponentBySerial =>
      'Añadir componente por número de serie (en stock)';

  @override
  String get addEquipment => 'Añadir equipo';

  @override
  String get addMember => 'Añadir miembro';

  @override
  String get addNote => 'Añadir nota';

  @override
  String get addNoteHint => 'Añadir una nota interna';

  @override
  String get addPart => 'Añadir pieza';

  @override
  String get addPhone => 'Añadir teléfono';

  @override
  String get addPhoto => 'Añadir foto';

  @override
  String get addProduct => 'Añadir producto';

  @override
  String get addRebate => 'Añadir descuento';

  @override
  String get addTag => 'Añadir etiqueta';

  @override
  String get addTask => 'Añadir tarea';

  @override
  String addedBy(String date, String name) {
    return 'añadido el $date por $name';
  }

  @override
  String get addressForm => 'Forma de trato';

  @override
  String get addressFormal => 'Formal (usted)';

  @override
  String get addressInformal => 'Informal (tú)';

  @override
  String get addVisit => 'Añadir visita';

  @override
  String get agreed => 'Acordado';

  @override
  String get all => 'Todos';

  @override
  String get allCategories => 'Todas las categorías';

  @override
  String get allOwners => 'Todos los responsables';

  @override
  String get allProducts => 'Todos los productos';

  @override
  String get allStatuses => 'Todos los estados';

  @override
  String get allTags => 'Todas las etiquetas';

  @override
  String get allTeams => 'Todos los equipos';

  @override
  String get always => 'siempre';

  @override
  String andMore(int count) {
    return '… y $count más';
  }

  @override
  String get announceRepair => 'Anunciar una reparación';

  @override
  String get announcedNotReceived => 'Anunciada, aún no recibida';

  @override
  String answerFallback(String ticketLanguage, String usedLanguage) {
    return 'Aún no hay texto en $ticketLanguage: se ha usado el texto en $usedLanguage. Tradúzcalo antes de enviarlo.';
  }

  @override
  String get anyValue => 'Cualquiera';

  @override
  String get appLanguage => 'Idioma';

  @override
  String get appTitle => 'tradocraft';

  @override
  String get apply => 'Aplicar';

  @override
  String get assignTo => 'Asignar a';

  @override
  String get attachFile => 'Adjuntar archivo';

  @override
  String get attachments => 'Archivos adjuntos';

  @override
  String get attachmentsHelp => 'Imágenes (JPEG, PNG, WebP) y PDF, hasta 12 MB';

  @override
  String get automatic => 'Automático';

  @override
  String get automationHelp =>
      'Las reglas se ejecutan en este orden cuando se crea o modifica un ticket o responde el cliente. Una regla se aplica si se cumplen todas sus condiciones. Las reglas solo modifican tickets; no envían correo.';

  @override
  String get automationRules => 'Reglas de automatización';

  @override
  String get autoRenew => 'Renovar automáticamente';

  @override
  String get backIntoWork => 'Volver a trabajar';

  @override
  String get backToCurrentRevision => 'Volver a la revisión actual';

  @override
  String get backlogByOwner => 'Tickets abiertos por responsable';

  @override
  String get backlogByPriority => 'Tickets abiertos por prioridad';

  @override
  String get backlogByStatus => 'Tickets abiertos por estado';

  @override
  String get backlogByTeam => 'Tickets abiertos por equipo';

  @override
  String get basisBusiness => 'horario laboral';

  @override
  String get basisElapsed => 'tiempo transcurrido';

  @override
  String get billOfMaterials => 'Lista de materiales';

  @override
  String get billable => 'Facturable';

  @override
  String get bomActive => 'activa';

  @override
  String get bomObsolete => 'obsoleta';

  @override
  String get bomRev => 'Rev. LdM';

  @override
  String get bomRevision => 'Revisión de la LdM';

  @override
  String get breached => 'Incumplidos';

  @override
  String get breachesByTeam => 'Incumplimientos del SLA por equipo';

  @override
  String get builtInto => 'Montado en';

  @override
  String get bulkKeepsQuotes =>
      'Los presupuestos anteriores a esta fecha mantienen los precios actuales';

  @override
  String get bulkPriceChange => 'Cambio de precios en bloque';

  @override
  String get businessHours => 'Horario laboral';

  @override
  String get businessHoursEnabled => 'Contar solo el horario laboral';

  @override
  String get businessHoursHelp =>
      'Los plazos de servicio solo cuentan las horas indicadas, en días no festivos. Desactivado, corren las 24 horas.';

  @override
  String get businessHoursInvalidTime => 'Introduzca las horas como HH:MM.';

  @override
  String get businessHoursRecompute =>
      'Al guardar se recalculan los plazos de todos los tickets abiertos.';

  @override
  String get businessHoursSaved =>
      'Guardado. Se recalcularon los tickets abiertos.';

  @override
  String get byStage => 'según la fase';

  @override
  String get callIncoming => 'Llamada entrante';

  @override
  String get callLogged => 'Llamada registrada';

  @override
  String get callMinutes => 'Duración (minutos)';

  @override
  String get callOutgoing => 'Llamada saliente';

  @override
  String get callSummary => 'Qué se habló';

  @override
  String callWith(String name) {
    return 'Llamada con $name';
  }

  @override
  String get canStillKeep => 'Aún puede mantener los precios acordados.';

  @override
  String get cancel => 'Cancelar';

  @override
  String get cancelJob => 'Cancelar trabajo';

  @override
  String get cancelOrder => 'Cancelar pedido';

  @override
  String get cancelReasonLabel => 'Motivo de la cancelación';

  @override
  String get cancelRepair => 'Cancelar reparación';

  @override
  String get cancelled => 'cancelado';

  @override
  String cancelledReason(String reason) {
    return 'Cancelado: $reason';
  }

  @override
  String get cannotConfirmNewer =>
      'Se está preparando una revisión más reciente de este pedido. Podrá confirmarla cuando se le haya enviado.';

  @override
  String get cannotConfirmOther => 'Este pedido no se puede confirmar.';

  @override
  String get captureSignature => 'Capturar firma';

  @override
  String get category => 'Categoría';

  @override
  String get categoryComplaint => 'reclamación';

  @override
  String get categoryLead => 'prospecto (nueva solicitud)';

  @override
  String get categoryOrderConfirmation => 'confirmación de pedido';

  @override
  String get categoryOther => 'otro';

  @override
  String get categoryProductionQuestion => 'consulta de producción';

  @override
  String get categoryQuestion => 'consulta';

  @override
  String get categoryQuoteChange => 'cambio de presupuesto';

  @override
  String get categoryService => 'servicio / reparación';

  @override
  String get cc => 'CC';

  @override
  String get change => 'Cambio';

  @override
  String get changePriority => 'Cambiar prioridad';

  @override
  String get channel => 'Canal';

  @override
  String get channelEmail => 'Correo';

  @override
  String get channelInternal => 'Pregunta interna';

  @override
  String get channelManual => 'Introducido a mano';

  @override
  String get channelPhone => 'Teléfono';

  @override
  String get channelPortal => 'Portal';

  @override
  String get checklistOnePerLine =>
      'Lista de comprobación (una tarea por línea)';

  @override
  String get chooseCustomerAndLine =>
      'Elija un cliente y añada al menos una línea válida.';

  @override
  String get choosePhoto => 'Elegir de la galería';

  @override
  String get chooseRebateTarget =>
      'Elija un producto o familia e indique un porcentaje.';

  @override
  String get chooseZonePrice => 'Elija una zona e indique un precio.';

  @override
  String get city => 'Ciudad';

  @override
  String get clear => 'Borrar';

  @override
  String get close => 'Cerrar';

  @override
  String get closedLabel => 'cerrados';

  @override
  String get closes => 'Cierra';

  @override
  String get closureKindCollectiveLeave => 'Vacaciones colectivas';

  @override
  String get closureKindHoliday => 'Festivo';

  @override
  String get closureKindInventory => 'Inventario';

  @override
  String get closureKindLabel => 'Motivo';

  @override
  String get closureKindOther => 'Otro';

  @override
  String get closureKindWorks => 'Obras';

  @override
  String get closurePolicyBlock => 'No permitido';

  @override
  String get closurePolicyBlockHelp =>
      'No se puede planificar una entrega un día de cierre; elija otra fecha.';

  @override
  String get closurePolicyOverride =>
      'Requiere un motivo y el derecho a anular';

  @override
  String get closurePolicyOverrideHelp =>
      'Solo los usuarios con derecho a anular pueden planificar un día de cierre, con un motivo.';

  @override
  String get closurePolicyTitle =>
      'Cuando se planifica una entrega un día en que el cliente está cerrado';

  @override
  String get closurePolicyWarn => 'Solo avisar';

  @override
  String get closurePolicyWarnHelp =>
      'La fecha se guarda y el cierre se muestra como aviso.';

  @override
  String get closures => 'Días de cierre';

  @override
  String get closuresShowPast => 'Mostrar cierres pasados';

  @override
  String get code => 'Código';

  @override
  String get codeNameRequired => 'El código y el nombre son obligatorios.';

  @override
  String get codeNameVatRequired =>
      'El código, el nombre y el tipo de IVA son obligatorios.';

  @override
  String get completeJob => 'Completar trabajo';

  @override
  String get completionNotes => 'Qué se hizo (y qué queda pendiente)';

  @override
  String get component => 'Componente';

  @override
  String get componentHistory => 'Historial de componentes';

  @override
  String componentSince(String product, String date) {
    return '$product · desde $date';
  }

  @override
  String get componentsAsBuilt => 'Componentes (según fabricación)';

  @override
  String get confirm => 'confirmar';

  @override
  String get confirmOnBehalf => 'Confirmar en nombre del cliente';

  @override
  String get confirmOnBehalfHelp =>
      'Registre que el cliente ha confirmado esta revisión, por ejemplo por teléfono o por correo electrónico.';

  @override
  String get confirmOrder => 'Confirmar pedido';

  @override
  String get confirmOrderHelp =>
      'Al confirmar, acepta este pedido tal como se muestra.';

  @override
  String get confirmation => 'Confirmación';

  @override
  String get confirmationDate => 'Confirmado el';

  @override
  String get confirmed => 'confirmado';

  @override
  String get confirmedRevision => 'Revisión confirmada';

  @override
  String get confirmedVia => 'Confirmado mediante';

  @override
  String get confirmedViaPortal => 'portal de clientes';

  @override
  String get confirmedViaStaff => 'empleado, en nombre del cliente';

  @override
  String conflictsOnDate(int count) {
    return '$count ya tienen un precio en esta fecha';
  }

  @override
  String get contactName => 'Nombre del contacto';

  @override
  String get contactPerson => 'Persona de contacto';

  @override
  String get contacts => 'Contactos';

  @override
  String get contractEnded => 'Finalizado';

  @override
  String get contractJobs => 'Intervenciones de este contrato';

  @override
  String contractLabel(String number) {
    return 'Contrato $number';
  }

  @override
  String contractOpenJobs(String count) {
    return '$count intervenciones abiertas';
  }

  @override
  String get contracts => 'Contratos';

  @override
  String get contractSuspended => 'Suspendido';

  @override
  String get convertHelp =>
      '¿El cliente quiere cambios? Cree primero una nueva revisión del presupuesto, vincúlela aquí y luego conviértala.';

  @override
  String get convertToOrder => 'Convertir en pedido';

  @override
  String get countAddLine => 'Añadir existencias no listadas';

  @override
  String get countCancel => 'Cancelar inventario';

  @override
  String get countCounted => 'Contado';

  @override
  String get countDifference => 'Diferencia';

  @override
  String get countExpected => 'Esperado';

  @override
  String get countInvalidQuantity => 'Una cantidad contada no es un número.';

  @override
  String countMovedDuring(int count) {
    return '$count línea(s) tuvieron movimientos durante el recuento; la cantidad contada los sustituyó.';
  }

  @override
  String get countNew => 'Nuevo inventario';

  @override
  String countOf(int count, int total) {
    return '$count de $total';
  }

  @override
  String get countOneProduct => 'Solo un producto (opcional)';

  @override
  String get countOverwriteWarning =>
      'Los movimientos hechos durante el recuento se sobrescriben: al contabilizar el inventario, la cantidad contada pasa a ser el stock.';

  @override
  String get countPost => 'Contabilizar inventario';

  @override
  String get countPostConfirm =>
      'Las diferencias se contabilizan como ajustes de stock. Las líneas sin cantidad contada no se modifican.';

  @override
  String get countProgress => 'Contado';

  @override
  String get countStart => 'Iniciar inventario';

  @override
  String get countStatusCounting => 'En curso';

  @override
  String get countStatusPosted => 'Contabilizado';

  @override
  String countStockAtPost(String quantity) {
    return 'Stock al contabilizar: $quantity';
  }

  @override
  String get countWarehouseRequired => 'Elija un almacén.';

  @override
  String get countWholeWarehouse => 'Todo el almacén';

  @override
  String get countedBy => 'Iniciado por';

  @override
  String get country => 'País';

  @override
  String get create => 'Crear';

  @override
  String get createLead => 'Crear oportunidad';

  @override
  String get createRepair => 'Crear reparación';

  @override
  String get created => 'creados';

  @override
  String get csatAlready => 'Esta encuesta ya fue respondida.';

  @override
  String get csatAverage => 'Satisfacción del cliente';

  @override
  String get csatBad => 'No satisfecho';

  @override
  String get csatComment => 'Comentario (opcional)';

  @override
  String get csatEnabled =>
      'Enviar una encuesta de satisfacción al cerrar un ticket';

  @override
  String get csatExpired => 'Este enlace ha caducado.';

  @override
  String get csatGood => 'Satisfecho';

  @override
  String get csatNone => 'Aún sin valoración';

  @override
  String csatQuestion(String number) {
    return '¿Está satisfecho con la gestión del ticket $number?';
  }

  @override
  String csatResponses(int count) {
    return '$count respuestas';
  }

  @override
  String get csatThanks => '¡Gracias por sus comentarios!';

  @override
  String get csatTitle => 'Satisfacción del cliente';

  @override
  String get csvCopied => 'CSV copiado al portapapeles';

  @override
  String get current => 'Actual';

  @override
  String get customer => 'Cliente';

  @override
  String get customerDeclinedToSign => 'El cliente se negó a firmar';

  @override
  String get customerEquipment => 'Equipos del cliente';

  @override
  String get customerLanguage => 'Idioma del cliente';

  @override
  String customerMainNumber(String name) {
    return 'Número principal de $name';
  }

  @override
  String customerNamed(String name) {
    return 'Cliente: $name';
  }

  @override
  String get customerOptional => 'Cliente (opcional)';

  @override
  String get customerReference => 'Referencia del cliente';

  @override
  String get customers => 'Clientes';

  @override
  String get customerSignature => 'Firma del cliente';

  @override
  String get date => 'Fecha';

  @override
  String declinedToSignWithReason(String reason) {
    return 'Se negó a firmar: $reason';
  }

  @override
  String get defaultTeam => 'Equipo predeterminado para tickets nuevos';

  @override
  String get deleteRuleConfirm =>
      '¿Eliminar esta regla? Los tickets que modificó conservan sus cambios.';

  @override
  String get deliveries => 'Entregas';

  @override
  String get deliveryBlocked =>
      'No se permiten entregas en un día de cierre. Elija otra fecha.';

  @override
  String get deliveryCancel => 'Cancelar entrega';

  @override
  String get deliveryCancelReason => 'Motivo de la cancelación';

  @override
  String get deliveryChooseSerial => 'Elegir n.º de serie';

  @override
  String deliveryClosedWarning(String from, String to, String next) {
    return 'El cliente está cerrado del $from al $to. Próximo día abierto: $next.';
  }

  @override
  String get deliveryFailed => 'no enviado';

  @override
  String get deliveryLocation => 'Ubicación';

  @override
  String get deliveryLot => 'Lote';

  @override
  String get deliveryNew => 'Nueva entrega';

  @override
  String get deliveryNoDate => 'sin fecha';

  @override
  String get deliveryNoLines =>
      'Nada que entregar todavía: reserve primero existencias para el pedido.';

  @override
  String get deliveryOnClosure => 'en un cierre';

  @override
  String get deliveryOverrideNeedsRight =>
      'Entregar un día de cierre requiere un motivo y el derecho a anularlo.';

  @override
  String get deliveryOverrideReason => 'Motivo para entregar un día de cierre';

  @override
  String get deliveryPicking => 'Lista de preparación';

  @override
  String get deliveryPlannedDate => 'Fecha prevista';

  @override
  String get deliveryQueued => 'pendiente de envío';

  @override
  String get deliverySent => 'enviado';

  @override
  String get deliverySerial => 'N.º de serie';

  @override
  String get deliverySettingsTitle => 'Ajustes de entrega';

  @override
  String get deliveryShip => 'Enviar';

  @override
  String get deliveryShipTo => 'Dirección de entrega';

  @override
  String get deliveryShippedSnack => 'Entrega enviada';

  @override
  String get deliveryStatusPlanned => 'planificada';

  @override
  String get deliveryStatusShipped => 'enviada';

  @override
  String deliveryUseDay(String date) {
    return 'Usar $date';
  }

  @override
  String get department => 'Departamento';

  @override
  String get departmentCode => 'Código';

  @override
  String get departmentMembers => 'Compañeros de este departamento';

  @override
  String get departments => 'Departamentos';

  @override
  String get departmentsHelp =>
      'Las reparaciones se envían a estos departamentos. Solo los compañeros de un departamento pueden trabajar en sus reparaciones.';

  @override
  String get description => 'Descripción';

  @override
  String get devModeHelp => 'El servidor debe ejecutarse con AUTH_MODE=dev';

  @override
  String get developmentUser => 'Usuario de desarrollo';

  @override
  String get deviceLanguage => 'Idioma del dispositivo';

  @override
  String get difference => 'Diferencia';

  @override
  String get dismiss => 'Rechazar';

  @override
  String get dismissRequest => 'Rechazar solicitud';

  @override
  String get draftSaved => 'Borrador guardado';

  @override
  String get dryRun => 'Probar en un ticket';

  @override
  String get dryRunHelp =>
      'Muestra qué reglas se aplicarían a un ticket y qué cambiarían. No se modifica nada.';

  @override
  String get dryRunRun => 'Ejecutar prueba';

  @override
  String get drawSignatureFirst => 'Dibuje primero una firma';

  @override
  String dueOnDate(String date) {
    return 'Para el $date';
  }

  @override
  String get edit => 'Editar';

  @override
  String get editClosure => 'Editar cierre';

  @override
  String get editContact => 'Editar contacto';

  @override
  String get editDepartment => 'Editar departamento';

  @override
  String get editJob => 'Editar trabajo';

  @override
  String get email => 'Correo electrónico';

  @override
  String get emailAddresses =>
      'Direcciones de correo (la primera es la principal)';

  @override
  String get endContract => 'Finalizar contrato';

  @override
  String get endDate => 'Fecha de fin';

  @override
  String get enterChange => 'Indique el cambio, p. ej. 4,5 para +4,5 %.';

  @override
  String enterValidNumbersLine(int line) {
    return 'Línea $line: indique números válidos.';
  }

  @override
  String get equipment => 'Equipo';

  @override
  String get estimatedMinutes => 'Minutos estimados';

  @override
  String get expectedClose => 'Cierre previsto';

  @override
  String expectedCloseDate(String date) {
    return 'Cierre previsto el $date';
  }

  @override
  String get expectedValue => 'Valor esperado';

  @override
  String get expectedValueEur => 'Valor esperado (€)';

  @override
  String get expectedValueOptional => 'Valor esperado (opcional)';

  @override
  String get explode => 'Desglosar';

  @override
  String get exportCsv => 'Copiar como CSV';

  @override
  String get families => 'Familias';

  @override
  String get family => 'Familia';

  @override
  String familyNamed(String code) {
    return 'Familia $code';
  }

  @override
  String get faultDescription => 'Qué falla';

  @override
  String get firstDueDate => 'Primer vencimiento';

  @override
  String get firstName => 'Nombre';

  @override
  String get firstResponseMinutes => 'Primera respuesta (min)';

  @override
  String get firstResponseTime => 'Primera respuesta';

  @override
  String get from => 'Desde';

  @override
  String get fromYou => 'Usted';

  @override
  String generalRebateInfo(String percent) {
    return 'Descuento general en todos los productos: $percent % (los productos con precio neto nunca tienen descuento).';
  }

  @override
  String get generalRebatePercent => 'Descuento general %';

  @override
  String greetingPreview(String greeting) {
    return 'Saludo: $greeting';
  }

  @override
  String get helpdesk => 'Atención al cliente';

  @override
  String get helpdeskReports => 'Informes';

  @override
  String get helpdeskSettings => 'Ajustes del helpdesk';

  @override
  String get history => 'Historial';

  @override
  String get holidayDate => 'Fecha';

  @override
  String get holidayInvalid => 'Introduzca una fecha (AAAA-MM-DD) y un nombre.';

  @override
  String get holidays => 'Festivos';

  @override
  String get holidaysHelp => 'Días en los que el helpdesk está cerrado.';

  @override
  String get calendarFor => 'Calendario de';

  @override
  String get calendarCompanyWide => 'Toda la empresa';

  @override
  String get calendarInherited =>
      'Este equipo no tiene calendario propio y usa el de la empresa. Al guardar se crea uno para el equipo.';

  @override
  String get calendarHolidaysInherited =>
      'Los festivos de la empresa también se aplican a este equipo. Los festivos añadidos aquí son propios del equipo.';

  @override
  String get calendarUseCompany => 'Usar el calendario de la empresa';

  @override
  String get calendarRemoved =>
      'El equipo vuelve a usar el calendario de la empresa.';

  @override
  String get basisMixed =>
      'horario laboral o tiempo transcurrido, según el equipo';

  @override
  String get inactive => 'inactivo';

  @override
  String get inactiveContact => 'Ya no está activo (se ha ido)';

  @override
  String indicativeLine(String description) {
    return '$description (orientativo, por confirmar)';
  }

  @override
  String indicativeToConfirm(int count) {
    return '$count precio(s) orientativo(s) por confirmar antes de enviar el presupuesto: indique el precio acordado en el campo precio de tarifa.';
  }

  @override
  String get insertPreparedAnswer => 'Insertar respuesta predefinida';

  @override
  String get instructions => 'Instrucciones';

  @override
  String get internalNote => 'Nota interna';

  @override
  String internalQuestionHelp(String language) {
    return 'Pregunta interna. Pregunte al cliente en $language con una respuesta y después anote la respuesta para su compañero como nota interna.';
  }

  @override
  String get internalTicket => 'Ticket interno';

  @override
  String get internalTicketHelp =>
      'Una pregunta entre compañeros; no se envía nada al cliente.';

  @override
  String get interval => 'Intervalo';

  @override
  String get jobCancelled => 'Cancelado';

  @override
  String get jobCompleted => 'Completado';

  @override
  String jobCount(String count) {
    return '$count intervenciones';
  }

  @override
  String jobCreatedOnDate(String date) {
    return 'intervención creada el $date';
  }

  @override
  String get jobInProgress => 'En curso';

  @override
  String get jobKindInspection => 'Inspección';

  @override
  String get jobKindInstallation => 'Instalación';

  @override
  String get jobKindMaintenance => 'Mantenimiento';

  @override
  String get jobKindOther => 'Otro';

  @override
  String get jobKindRepair => 'Reparación';

  @override
  String get jobName => 'Título';

  @override
  String get jobPlanned => 'Planificado';

  @override
  String get jobTitle => 'Cargo';

  @override
  String get jobs => 'Trabajos';

  @override
  String get keepAgreedPrices => 'Mantener precios acordados';

  @override
  String get language => 'Idioma';

  @override
  String languageSameAsCustomer(String language) {
    return 'Igual que la empresa ($language)';
  }

  @override
  String lastDays(int days) {
    return 'Últimos $days días';
  }

  @override
  String get lastName => 'Apellidos';

  @override
  String get leadCreated => 'Oportunidad creada';

  @override
  String get leadDays => 'Crear la intervención (días antes)';

  @override
  String get lineDiscount => 'Dto. adic. ±';

  @override
  String get listPrice => 'Precio de tarifa';

  @override
  String get loadMore => 'Cargar más';

  @override
  String get logCall => 'Registrar llamada';

  @override
  String get logTime => 'Registrar tiempo';

  @override
  String get logWork => 'Registrar trabajo';

  @override
  String get loginLinkInvalid =>
      'Este enlace ha caducado o ya se usó. Solicite uno nuevo.';

  @override
  String get loginLinkSent =>
      'Si esta dirección puede usar el portal, le hemos enviado un enlace de acceso. Revise su bandeja de entrada.';

  @override
  String get lookUp => 'Buscar';

  @override
  String get lost => 'Perdida';

  @override
  String get lostReason => 'Motivo de pérdida';

  @override
  String get markAsLost => 'Marcar como perdida';

  @override
  String get markAsSent => 'Marcar como enviado';

  @override
  String get markCompleted => 'Marcar como completada';

  @override
  String get markReceived => 'Marcar como recibida';

  @override
  String get markShipped => 'Marcar como enviada';

  @override
  String get markWaitingCustomer => 'Esperando al cliente';

  @override
  String get markWaitingParts => 'Esperando piezas';

  @override
  String get materialRequirements =>
      'Necesidades de material (revisión activa)';

  @override
  String get mergeAction => 'Combinar';

  @override
  String get mergeHelp =>
      'Todos los mensajes, archivos, etiquetas y registros de tiempo pasan al ticket elegido del mismo cliente. Este ticket se cierra. No se puede deshacer.';

  @override
  String get mergeNoCandidates => 'No hay otro ticket abierto de este cliente.';

  @override
  String get mergeTicket => 'Combinar con otro ticket…';

  @override
  String mergedInto(String number) {
    return 'Combinado con $number';
  }

  @override
  String get message => 'Mensaje';

  @override
  String get messageSent => 'Mensaje enviado';

  @override
  String get minutes => 'Minutos';

  @override
  String minutesShort(String minutes) {
    return '$minutes min';
  }

  @override
  String get moduleDelivery => 'Entrega';

  @override
  String get moduleProduction => 'Producción';

  @override
  String get moveDown => 'Bajar';

  @override
  String get moveUp => 'Subir';

  @override
  String get months => 'Meses';

  @override
  String get myJobs => 'Mis trabajos';

  @override
  String get myOrders => 'Mis pedidos';

  @override
  String get myProfile => 'Mis datos';

  @override
  String get myRepairs => 'Mis reparaciones';

  @override
  String get myTickets => 'Mis tickets';

  @override
  String get name => 'Nombre';

  @override
  String get navigate => 'Navegar';

  @override
  String get netPrice => 'Precio neto';

  @override
  String get newAnswer => 'Nueva respuesta';

  @override
  String get newContact => 'Nuevo contacto';

  @override
  String get newContract => 'Nuevo contrato';

  @override
  String get newCustomer => 'Nuevo cliente';

  @override
  String get newDepartment => 'Nuevo departamento';

  @override
  String get newFamily => 'Nueva familia';

  @override
  String get newItem => 'Nuevo';

  @override
  String get newJob => 'Nuevo trabajo';

  @override
  String get newLabel => 'Nuevo';

  @override
  String get newOpportunity => 'Nueva oportunidad';

  @override
  String get newPrice => 'Nuevo precio';

  @override
  String newPricesValidFrom(String date) {
    return 'Nuevos precios válidos desde $date';
  }

  @override
  String get newPriority => 'Nueva prioridad';

  @override
  String get newProduct => 'Nuevo producto';

  @override
  String get newQuotation => 'Nuevo presupuesto';

  @override
  String get newRepair => 'Nueva reparación';

  @override
  String get newRevision => 'Nueva revisión';

  @override
  String newRevisionOf(String number) {
    return 'Nueva revisión de $number';
  }

  @override
  String get newRule => 'Nueva regla';

  @override
  String get newTeam => 'Nuevo equipo';

  @override
  String get newTicket => 'Nuevo ticket';

  @override
  String get no => 'No';

  @override
  String get noAccess =>
      'Su cuenta aún no tiene acceso a ninguna sección. Consulte a un administrador.';

  @override
  String get noActiveBom => 'No hay lista de materiales activa.';

  @override
  String get noBom => 'Aún no hay lista de materiales.';

  @override
  String get noClosures => 'No hay cierres registrados.';

  @override
  String get noContactLinked => 'Ninguna persona de contacto vinculada';

  @override
  String get noContacts => 'Aún no hay contactos.';

  @override
  String get noContracts => 'Sin contratos';

  @override
  String get noCustomerEquipment => 'Este cliente no tiene equipos instalados';

  @override
  String get noCustomerLinked => 'Ningún cliente vinculado';

  @override
  String get noEquipment => 'Ningún equipo vinculado';

  @override
  String get noExpectedClose => 'Sin fecha de cierre prevista';

  @override
  String get noJobsAssigned => 'No tiene trabajos asignados.';

  @override
  String get noLines => 'Aún no hay líneas.';

  @override
  String get noOrdersYet => 'Aún no tiene pedidos.';

  @override
  String get noParts => 'Ninguna pieza registrada';

  @override
  String get noPermission => 'No tiene permiso para cambiar estos ajustes.';

  @override
  String get noPhotos => 'Aún no hay fotos.';

  @override
  String get noPrices => 'Aún no hay precios.';

  @override
  String get noPriorityChanges => 'Aún no hay cambios de prioridad.';

  @override
  String get noPriorityRequests => 'Sin solicitudes.';

  @override
  String get noQuotations => 'Aún no hay presupuestos.';

  @override
  String get noRebates => 'Sin descuentos por producto o familia.';

  @override
  String get noRepairs => 'Sin reparaciones.';

  @override
  String get noRepairsYet => 'Aún no tiene reparaciones.';

  @override
  String get noRules => 'Aún no hay reglas.';

  @override
  String get noSerialComponents => 'Sin componentes con número de serie.';

  @override
  String get noSignature => 'Aún sin firmar';

  @override
  String get noTeam => 'Sin equipo';

  @override
  String get noTechnicians => 'Nadie asignado';

  @override
  String get noTicketsYet => 'Aún no tiene tickets.';

  @override
  String get noTimeEntries => 'Aún no hay tiempo registrado.';

  @override
  String get nobodyFound => 'No se encontró a nadie con este número.';

  @override
  String get nonBillable => 'No facturable';

  @override
  String get none => 'Ninguno';

  @override
  String get notClosed => 'No cerrados';

  @override
  String get notRouted => 'Aún sin enviar';

  @override
  String get notScheduled => 'Sin planificar';

  @override
  String get noteAdded => 'Nota añadida';

  @override
  String get noteOptional => 'Observación (opcional)';

  @override
  String get notes => 'Observaciones';

  @override
  String get noUpcomingVisits => 'Sin visitas previstas';

  @override
  String get number => 'Número';

  @override
  String get numberRequired => 'Número';

  @override
  String get obsoleteRevision => 'Revisión obsoleta (solo lectura).';

  @override
  String get oidcAccessToken => 'Token de acceso OIDC';

  @override
  String get ok => 'Aceptar';

  @override
  String get onSiteContact => 'Contacto en el lugar';

  @override
  String get openEnded => 'abierto';

  @override
  String get openJobsOnly => 'Solo activos';

  @override
  String get openOnly => 'Solo abiertas';

  @override
  String get openOpportunity => 'Abrir oportunidad';

  @override
  String openPriorityRequests(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count solicitudes de prioridad abiertas',
      one: '1 solicitud de prioridad abierta',
    );
    return '$_temp0';
  }

  @override
  String openRepairsCount(String count) {
    return '$count abiertas';
  }

  @override
  String get opens => 'Abre';

  @override
  String get opportunities => 'Oportunidades';

  @override
  String get opportunity => 'Oportunidad';

  @override
  String get opportunityOptional => 'Oportunidad (opcional)';

  @override
  String get order => 'Pedido';

  @override
  String orderAlreadyConfirmed(String date) {
    return 'Este pedido se confirmó el $date.';
  }

  @override
  String get orderAwaitingConfirmation => 'pendiente de su confirmación';

  @override
  String get orderConfirmationQueued =>
      'La confirmación de pedido está en cola para su envío.';

  @override
  String get orderConfirmedSnack => 'Pedido confirmado.';

  @override
  String get orderConfirmedThanks => 'Gracias, su pedido está confirmado.';

  @override
  String get orderCreated => 'Pedido creado';

  @override
  String get orderSaved => 'Pedido guardado.';

  @override
  String get orderStatusInPlanning => 'en planificación';

  @override
  String get orderStatusInProduction => 'en producción';

  @override
  String get orderStatusPartiallyShipped => 'enviada parcialmente';

  @override
  String get orderStatusPlanned => 'planificado';

  @override
  String get orderStatusReadyForShipping => 'listo para el envío';

  @override
  String get orderStatusShipped => 'enviada';

  @override
  String get orderStockRelease => 'Liberar';

  @override
  String get orderStockReserve => 'Reservar';

  @override
  String get orderStockReserveHint =>
      'El stock se puede reservar cuando el cliente confirme el pedido.';

  @override
  String orderStockThisLine(String quantity) {
    return '$quantity reservado para esta línea';
  }

  @override
  String get orderStockTitle => 'Disponibilidad de stock';

  @override
  String get orders => 'Pedidos';

  @override
  String get owner => 'Responsable';

  @override
  String get ownerEmpty => 'Nadie';

  @override
  String get ownerSet => 'Alguien';

  @override
  String get p90 => '90 % dentro de';

  @override
  String get partFreeTextHint => 'Elija un producto o escriba una descripción';

  @override
  String get partsUsed => 'Piezas utilizadas';

  @override
  String get paymentDays => 'Plazo de pago (días)';

  @override
  String get pdf => 'PDF';

  @override
  String get perDay => 'Por día';

  @override
  String get perWeek => 'Por semana';

  @override
  String get personTitle => 'Tratamiento';

  @override
  String get phone => 'Teléfono';

  @override
  String get phoneCall => 'Llamada telefónica';

  @override
  String get phoneDirect => 'Directo';

  @override
  String get phoneHint => 'Con prefijo, p. ej. +32 475 12 34 56';

  @override
  String get phoneMobile => 'Móvil';

  @override
  String get phoneNumber => 'Número de teléfono';

  @override
  String get phoneNumbers => 'Teléfonos';

  @override
  String get phoneOther => 'Otro';

  @override
  String get phoneWork => 'Trabajo';

  @override
  String photoBy(String name) {
    return 'por $name';
  }

  @override
  String get photos => 'Fotos';

  @override
  String get photosAdded => 'Fotos añadidas';

  @override
  String get placeholdersHelp =>
      'Marcadores (clic para insertar en el idioma actual):';

  @override
  String get portalAccess => 'Acceso al portal de clientes';

  @override
  String get portalScope => 'Ve en el portal';

  @override
  String get portalScopeCustomer => 'Todos los tickets de la empresa';

  @override
  String get portalScopeOwn => 'Solo sus tickets';

  @override
  String get portalSignInIntro =>
      'Introduzca su correo electrónico. Le enviaremos un enlace para acceder, sin contraseña.';

  @override
  String get portalTitle => 'Portal de clientes';

  @override
  String get postalCode => 'Código postal';

  @override
  String get preparedAnswers => 'Respuestas predefinidas';

  @override
  String get preparedAnswersHelp =>
      'Respuestas reutilizables a preguntas frecuentes, un texto por idioma. Los marcadores como «quotation.number» se rellenan para el ticket; el texto puede editarse antes de enviarlo.';

  @override
  String get preview => 'Vista previa';

  @override
  String priceListOf(String date) {
    return 'Tarifa del $date';
  }

  @override
  String get priceListValidOn => 'Tarifa válida el';

  @override
  String get priceZone => 'Zona de precios';

  @override
  String pricedOnPriceListOf(String date) {
    return 'Valorado con la tarifa del $date';
  }

  @override
  String get prices => 'Precios';

  @override
  String pricesCreated(int count, String date) {
    return '$count precios creados, válidos desde $date';
  }

  @override
  String pricesWillChange(int count) {
    return 'Cambiarán $count precios';
  }

  @override
  String get pricing => 'Precios';

  @override
  String get pricingIndicative => 'orientativo';

  @override
  String get pricingIndicativeHelp =>
      'Orientativo (confirmar en cada presupuesto)';

  @override
  String get pricingList => 'precio de tarifa';

  @override
  String get pricingListHelp => 'Precio de tarifa (se aplican descuentos)';

  @override
  String get pricingNet => 'neto';

  @override
  String get pricingNetHelp => 'Precio neto (nunca con descuento)';

  @override
  String get priority => 'Prioridad';

  @override
  String priorityChangeLine(String date, String from, String to, String name) {
    return '$date: $from → $to por $name';
  }

  @override
  String get priorityChangedSnack => 'Prioridad cambiada.';

  @override
  String get priorityExpedite => 'urgente';

  @override
  String get priorityHigh => 'alta';

  @override
  String get priorityHistory => 'Historial de prioridad';

  @override
  String get priorityLow => 'baja';

  @override
  String get priorityNormal => 'normal';

  @override
  String get priorityRequestHelp =>
      'Nuestro equipo decide si se cambia la prioridad y cómo.';

  @override
  String priorityRequestLine(String name, String priority) {
    return '$name solicita la prioridad: $priority';
  }

  @override
  String get priorityRequestSent => 'Su solicitud ha sido enviada.';

  @override
  String get priorityRequests => 'Solicitudes de prioridad del cliente';

  @override
  String get priorityUrgent => 'urgente';

  @override
  String get probability => 'Probabilidad';

  @override
  String get probabilityPercent => 'Probabilidad %';

  @override
  String get probabilityShort => 'Prob.';

  @override
  String get product => 'Producto';

  @override
  String productCount(int count) {
    return '$count productos';
  }

  @override
  String get productDescription => 'Producto';

  @override
  String get productFamilies => 'Familias de productos';

  @override
  String get productFamily => 'Familia de productos';

  @override
  String get productRebates => 'Descuentos por producto';

  @override
  String get productTypeConfigurable => 'configurable';

  @override
  String get productTypeManufactured => 'fabricado';

  @override
  String get productTypePurchased => 'comprado';

  @override
  String get productTypeService => 'servicio';

  @override
  String get production => 'Producción';

  @override
  String productionBomRevision(String revision) {
    return 'Revisión de lista de materiales $revision';
  }

  @override
  String get productionBuildComponent => 'Fabricar';

  @override
  String get productionBuildForStock => 'Fabricar para stock';

  @override
  String get productionCancel => 'Cancelar orden de producción';

  @override
  String get productionCancelReason => 'Motivo de la cancelación';

  @override
  String get productionChooseSerials => 'Elegir números de serie';

  @override
  String get productionComplete => 'Completar';

  @override
  String get productionCompleted => 'Orden de producción completada';

  @override
  String get productionComponents => 'Componentes';

  @override
  String get productionConsumed => 'Consumido';

  @override
  String get productionDivision => 'División';

  @override
  String get productionDivisionAdd => 'Añadir división';

  @override
  String get productionDivisionEdit => 'Editar división';

  @override
  String get productionDivisions => 'Divisiones';

  @override
  String get productionExpiry => 'Fecha de caducidad';

  @override
  String get productionExtraFrom => 'Existencias adicionales de';

  @override
  String get productionLocation => 'Almacenar en';

  @override
  String get productionLot => 'Número de lote';

  @override
  String get productionNoDivision => 'sin división';

  @override
  String get productionNothingToRelease =>
      'Nada que liberar: cada línea a fabricar está cubierta por órdenes de producción.';

  @override
  String get productionOrderCancelled =>
      'El pedido de venta fue cancelado. Cancele esta orden de producción para liberar sus componentes.';

  @override
  String get productionOrdersOfOrder => 'Órdenes de producción';

  @override
  String get productionOutput => 'Resultado';

  @override
  String get productionPlannedReady => 'Prevista lista';

  @override
  String get productionPromised => 'Prometido';

  @override
  String get productionQuantityBuilt => 'Cantidad fabricada';

  @override
  String get productionRelease => 'Liberar a producción';

  @override
  String get productionReleased => 'Liberado a producción';

  @override
  String get productionRequired => 'Necesario';

  @override
  String get productionReserveAgain => 'Reservar de nuevo';

  @override
  String get productionReserved => 'Reservado';

  @override
  String get productionScrap => 'Desecho';

  @override
  String get productionSerials => 'Números de serie, uno por línea';

  @override
  String productionSerialsNeeded(String count) {
    return 'Elija $count números de serie';
  }

  @override
  String get productionShort => 'falta de existencias';

  @override
  String get productionStart => 'Iniciar';

  @override
  String get productionStatusCompleted => 'completada';

  @override
  String get productionStatusInProgress => 'en curso';

  @override
  String get productionStatusReleased => 'liberada';

  @override
  String get productionToRelease => 'Por liberar';

  @override
  String get products => 'Productos';

  @override
  String get profileHelp =>
      'Elija el idioma y el trato que usamos en nuestros correos y en este portal.';

  @override
  String get quantity => 'Cantidad';

  @override
  String get question => 'Pregunta';

  @override
  String get questionToSales => 'Pregunta a ventas';

  @override
  String questionToSalesHelp(String salesperson) {
    return '$salesperson recibe esta pregunta y la plantea al cliente en su idioma.';
  }

  @override
  String get quotation => 'Presupuesto';

  @override
  String get quotations => 'Presupuestos';

  @override
  String raisedBy(String name) {
    return 'Creado por $name';
  }

  @override
  String get reason => 'Motivo';

  @override
  String get reasonRequired => 'El motivo es obligatorio.';

  @override
  String get rebate => 'Dto. estándar';

  @override
  String rebateFor(String target, String percent) {
    return '$target: $percent %';
  }

  @override
  String get rebatePercent => 'Descuento %';

  @override
  String get rebateSourceFamily => 'familia';

  @override
  String get rebateSourceGeneral => 'general';

  @override
  String get rebateSourceProduct => 'producto';

  @override
  String rebatesOf(String customer) {
    return 'Descuentos · $customer';
  }

  @override
  String get recurrenceMonthList => 'En los meses elegidos';

  @override
  String get recurrenceMonths => 'Cada N meses';

  @override
  String get recurrenceWeeks => 'Cada N semanas';

  @override
  String registerProduct(String product) {
    return 'Registrar $product';
  }

  @override
  String get registerUnit => 'Registrar unidad';

  @override
  String get registered => 'Registrado';

  @override
  String get rejected => 'Rechazado';

  @override
  String get remove => 'Eliminar';

  @override
  String get removeAndScrap => 'Quitar y desechar';

  @override
  String get removeBackToStock => 'Quitar, devolver al stock';

  @override
  String get removeLine => 'Eliminar línea';

  @override
  String removedBy(String date, String name) {
    return 'quitado el $date por $name';
  }

  @override
  String get reopenJob => 'Reabrir trabajo';

  @override
  String get repairAnnouncedByTicket =>
      'El cliente la anunció (el producto aún no ha llegado)';

  @override
  String get repairAnnouncedOk =>
      'Reparación anunciada. Escriba el número en el paquete y envíenos el producto.';

  @override
  String get repairPhotosHelp =>
      'Las fotos del producto y de la avería nos ayudan mucho.';

  @override
  String get repairPortalAnnounced => 'Anunciada, aún no recibida';

  @override
  String get repairPortalInRepair => 'En reparación';

  @override
  String get repairPortalReady => 'Reparada, lista para enviar';

  @override
  String get repairPortalWaitingCustomer => 'Esperando su respuesta';

  @override
  String get repairStatusAnnounced => 'Anunciada';

  @override
  String get repairStatusCancelled => 'Cancelada';

  @override
  String get repairStatusCompleted => 'Completada';

  @override
  String get repairStatusInProgress => 'En curso';

  @override
  String get repairStatusReceived => 'Recibida';

  @override
  String get repairStatusRouted => 'En cola del departamento';

  @override
  String get repairStatusShipped => 'Enviada de vuelta';

  @override
  String get repairStatusWaitingCustomer => 'Esperando al cliente';

  @override
  String get repairStatusWaitingParts => 'Esperando piezas';

  @override
  String get repairs => 'Reparaciones';

  @override
  String get replaceSignature => 'Sustituir firma';

  @override
  String get replyQueued => 'Respuesta en cola de envío';

  @override
  String get replyToCustomer => 'Responder al cliente';

  @override
  String get reportQueued => 'El informe se enviará';

  @override
  String get reportSend => 'Enviar informe por correo';

  @override
  String reportSentTo(String to, String date) {
    return 'Enviado a $to el $date';
  }

  @override
  String get reportView => 'Ver informe (PDF)';

  @override
  String repricingFailed(String error) {
    return 'No se han podido calcular los precios de hoy: $error';
  }

  @override
  String get requestDismissedSnack => 'Solicitud rechazada.';

  @override
  String get requestLessUrgent => 'Menos urgente';

  @override
  String get requestMoreUrgent => 'Más urgente (urgente)';

  @override
  String get requestPriorityChange => 'Solicitar cambio de prioridad';

  @override
  String get requestStatusApplied => 'aplicada';

  @override
  String get requestStatusDismissed => 'rechazada';

  @override
  String get requestStatusOpen => 'abierta';

  @override
  String get required => 'Obligatorio';

  @override
  String get resolutionHours => 'Resolución en (horas)';

  @override
  String get resolutionMinutes => 'Resolución (min)';

  @override
  String get resolutionTime => 'Resolución';

  @override
  String get responseTimes => 'Tiempos de respuesta y resolución';

  @override
  String get responseHours => 'Respuesta en (horas)';

  @override
  String get retry => 'Reintentar';

  @override
  String get revisionActivated => 'Revisión activada';

  @override
  String revisionItem(int revision, String status, int lines) {
    return 'Revisión $revision · $status · $lines líneas';
  }

  @override
  String get revisionNotes => 'Notas de la revisión';

  @override
  String revisionNumber(int revision) {
    return 'Revisión $revision';
  }

  @override
  String revisionPriceLists(String agreed, String current) {
    return 'Precios acordados: tarifa del $agreed. Precios actuales: tarifa del $current.';
  }

  @override
  String revisionViewNotice(int revision, int current) {
    return 'Está viendo la revisión $revision (solo lectura). La revisión actual es la $current.';
  }

  @override
  String get revisions => 'Revisiones';

  @override
  String get roleInvoicing => 'Facturación';

  @override
  String get roleManagement => 'Dirección';

  @override
  String get roleOther => 'Otro';

  @override
  String get rolePurchasing => 'Compras';

  @override
  String get roleQuality => 'Calidad';

  @override
  String get roleSalesContact => 'Contacto comercial';

  @override
  String get roleTechnical => 'Técnico';

  @override
  String get roles => 'Funciones';

  @override
  String get roundTo => 'Redondear a';

  @override
  String get routeNote => 'Nota para el departamento';

  @override
  String get routeRepair => 'Enviar a departamento';

  @override
  String get ruleActions => 'Acciones';

  @override
  String get ruleConditions => 'Condiciones (deben cumplirse todas)';

  @override
  String get ruleConditionsHelp =>
      'Elija los valores que quiera por línea; el ticket necesita uno. Las líneas vacías se ignoran.';

  @override
  String get ruleNameRequired => 'Una regla necesita un nombre.';

  @override
  String get ruleResultAfterStop => 'omitida (detenida)';

  @override
  String get ruleResultInactive => 'desactivada';

  @override
  String get ruleResultMatched => 'se aplica';

  @override
  String get ruleResultNoMatch => 'no coincide';

  @override
  String get ruleResultOtherTrigger => 'otro evento';

  @override
  String get ruleTrigger => 'Cuándo';

  @override
  String get ruleTriggerCreated => 'Se crea un ticket';

  @override
  String get ruleTriggerCustomerReplied => 'El cliente responde';

  @override
  String get ruleTriggerUpdated => 'El personal actualiza un ticket';

  @override
  String get salesperson => 'Comercial';

  @override
  String get save => 'Guardar';

  @override
  String get saveChanges => 'Guardar cambios';

  @override
  String get saveDraft => 'Guardar borrador';

  @override
  String get saved => 'Guardado';

  @override
  String savedNumber(String number) {
    return '$number guardado';
  }

  @override
  String get scheduledEnd => 'Fin';

  @override
  String get scheduledStart => 'Inicio';

  @override
  String get scrapPercent => 'Merma %';

  @override
  String get search => 'Buscar';

  @override
  String get send => 'Enviar';

  @override
  String get sendBackToTriage => 'Devolver a triaje';

  @override
  String get sendLoginLink => 'Enviar enlace de acceso';

  @override
  String get sendOrderConfirmation => 'Enviar confirmación de pedido';

  @override
  String get sendOrderConfirmationHelp =>
      'La confirmación de pedido se envía al cliente por correo electrónico en PDF. Después ya no se podrá editar la revisión.';

  @override
  String get senderDomain => 'Dominio del remitente';

  @override
  String get serial => 'n.º de serie';

  @override
  String get serialBuiltIn => 'montado';

  @override
  String get serialDelivered => 'entregado';

  @override
  String get serialInStock => 'en stock';

  @override
  String get serialInstalled => 'instalado';

  @override
  String get serialNumber => 'Número de serie';

  @override
  String get serialNumbers => 'Números de serie';

  @override
  String get serialNumbersHelp =>
      'Cada unidad tiene su propio número de serie e historial';

  @override
  String get serialScrapped => 'desechado';

  @override
  String get server => 'Servidor';

  @override
  String get serviceReport => 'Informe de servicio';

  @override
  String get setWarranty => 'Definir garantía';

  @override
  String get showInactive => 'Mostrar inactivos';

  @override
  String signedByOn(String name, String date) {
    return 'Firmado por $name el $date';
  }

  @override
  String get signerName => 'Nombre del firmante';

  @override
  String get signHere => 'Firme aquí';

  @override
  String get signIn => 'Iniciar sesión';

  @override
  String get signOut => 'Cerrar sesión';

  @override
  String signedInAs(String name) {
    return 'Conectado como $name';
  }

  @override
  String get signingIn => 'Accediendo…';

  @override
  String get singleProduct => 'Producto concreto';

  @override
  String get siteAddress => 'Dónde ir';

  @override
  String get siteName => 'Nombre del lugar';

  @override
  String skippedWithoutPrice(int count) {
    return '$count combinaciones producto/zona no tienen precio y se omiten';
  }

  @override
  String get sla => 'SLA';

  @override
  String get slaAndSatisfaction => 'SLA y satisfacción';

  @override
  String get slaAtRisk => 'En riesgo';

  @override
  String get slaBreached => 'Incumplido';

  @override
  String get slaCompliance => 'Cumplimiento del SLA';

  @override
  String slaComplianceDetail(int breached, int total) {
    return '$breached de $total incumplidos';
  }

  @override
  String slaDue(String date) {
    return 'Vence $date';
  }

  @override
  String get slaFilterAny => 'Cualquier estado SLA';

  @override
  String get slaFirstResponse => 'Primera respuesta';

  @override
  String slaLeft(String time) {
    return 'quedan $time';
  }

  @override
  String get slaMet => 'Cumplido';

  @override
  String get slaNone => 'Sin SLA';

  @override
  String get slaOnTime => 'A tiempo';

  @override
  String slaOverdue(String time) {
    return '$time de retraso';
  }

  @override
  String get slaPaused => 'Reloj detenido: esperando al cliente';

  @override
  String get slaPolicies => 'Niveles de servicio (SLA)';

  @override
  String get slaPoliciesHelp =>
      'Tiempo transcurrido real; no se tienen en cuenta las horas laborables. El reloj se detiene mientras un ticket espera al cliente.';

  @override
  String get slaResolution => 'Resolución';

  @override
  String get splitAction => 'Dividir';

  @override
  String splitFiles(int count) {
    return '$count archivo(s)';
  }

  @override
  String get splitHelp =>
      'Elija los mensajes que pasan a un ticket nuevo del mismo cliente. Sus archivos los acompañan. No se puede deshacer (los tickets se pueden volver a fusionar).';

  @override
  String get splitKeepOne =>
      'Al menos un mensaje debe quedarse en este ticket.';

  @override
  String get splitSubjectOptional => 'Asunto del ticket nuevo (opcional)';

  @override
  String get splitTicket => 'Dividir ticket…';

  @override
  String get stage => 'Fase';

  @override
  String get stageLead => 'prospecto';

  @override
  String get stageLost => 'perdida';

  @override
  String get stageNegotiation => 'negociación';

  @override
  String get stageProposal => 'propuesta';

  @override
  String get stageQualified => 'cualificada';

  @override
  String get stageWon => 'ganada';

  @override
  String get startDate => 'Fecha de inicio';

  @override
  String get startJob => 'Empezar trabajo';

  @override
  String get startWork => 'Empezar el trabajo';

  @override
  String get status => 'Estado';

  @override
  String get statusAccepted => 'aceptado';

  @override
  String get statusAfterwards => 'Estado después';

  @override
  String get statusDraft => 'borrador';

  @override
  String get statusRejected => 'rechazado';

  @override
  String get statusSent => 'enviado';

  @override
  String get statusSuperseded => 'sustituido';

  @override
  String get stopOnMatch => 'Parar tras esta regla';

  @override
  String get stopOnMatchHelp =>
      'Si se cumple, se omiten las reglas siguientes.';

  @override
  String get stockAdjust => 'Ajustar';

  @override
  String get stockAdjustReason => '¿Por qué se corrige el stock?';

  @override
  String get stockAdjustRequired =>
      'Elija una ubicación, introduzca una cantidad y un motivo.';

  @override
  String get stockAllWarehouses => 'Todos los almacenes';

  @override
  String get stockAssignedTo => 'Asignado a';

  @override
  String get stockAvailable => 'Disponible';

  @override
  String get stockBarcode => 'Código de barras';

  @override
  String get stockBelowReorder => 'Bajo el punto de pedido';

  @override
  String get stockChangeBy => 'Cambiar en (+/−)';

  @override
  String get stockCounts => 'Inventarios';

  @override
  String stockExpiredQty(String quantity) {
    return '$quantity caducado(s)';
  }

  @override
  String get stockExpiry => 'Fecha de caducidad';

  @override
  String get stockFrom => 'Desde';

  @override
  String get stockInVans => 'En furgonetas';

  @override
  String get stockInvalidNumbers =>
      'El coste, los niveles de pedido y el plazo deben ser números.';

  @override
  String get stockIssue => 'Entregar';

  @override
  String get stockKindVan => 'Furgoneta de servicio';

  @override
  String get stockKindWarehouse => 'Almacén';

  @override
  String get stockLeadTime => 'Plazo de entrega (días)';

  @override
  String get stockLocation => 'Ubicación';

  @override
  String get stockLocations => 'Existencias por ubicación';

  @override
  String get stockLot => 'Lote';

  @override
  String get stockLotNumber => 'Número de lote';

  @override
  String get stockLotTrackingHelp => 'Lotes';

  @override
  String get stockMoves => 'Movimientos';

  @override
  String get stockNewLocation => 'Nueva ubicación';

  @override
  String get stockNewLot => 'Lote nuevo';

  @override
  String get stockNewQuantity => 'Nueva cantidad';

  @override
  String get stockNewVan => 'Nueva furgoneta de servicio';

  @override
  String get stockNewWarehouse => 'Nuevo almacén';

  @override
  String get stockNoMoves => 'Aún no hay movimientos';

  @override
  String get stockNoWarehouses =>
      'Aún no hay almacenes. Un administrador los crea aquí.';

  @override
  String get stockNote => 'Nota';

  @override
  String get stockNothingInStock => 'Sin existencias';

  @override
  String get stockOnHand => 'En stock';

  @override
  String get stockOnlyInStock => 'Solo con existencias';

  @override
  String get stockProductQuantityRequired =>
      'Elija un producto e introduzca una cantidad.';

  @override
  String get stockReasonConsumption => 'Consumo';

  @override
  String get stockReasonCorrection => 'Corrección';

  @override
  String get stockReasonCount => 'Inventario';

  @override
  String get stockReasonManualReceipt => 'Recepción manual';

  @override
  String get stockReasonProductionOutput => 'Salida de producción';

  @override
  String get stockReasonPurchaseReceipt => 'Recepción de compra';

  @override
  String get stockReasonRepairUse => 'Usado en reparación';

  @override
  String get stockReasonReturn => 'Devolución de cliente';

  @override
  String get stockReasonReversal => 'Reversión';

  @override
  String get stockReasonScrap => 'Desecho';

  @override
  String get stockReasonShipment => 'Envío';

  @override
  String get stockReasonVanReplenishment => 'Reposición de furgoneta';

  @override
  String get stockReceive => 'Recibir';

  @override
  String get stockReorderMax => 'Máximo de pedido';

  @override
  String get stockReorderMin => 'Mínimo de pedido';

  @override
  String get stockReservations => 'Reservado para pedidos';

  @override
  String get stockReserved => 'Reservado';

  @override
  String get stockReverse => 'Revertir';

  @override
  String get stockReverseAnyway => 'Revertir de todos modos';

  @override
  String get stockReverseTitle => 'Revertir este movimiento';

  @override
  String get stockReversed => 'revertido';

  @override
  String stockShortBy(String quantity) {
    return 'Faltan $quantity';
  }

  @override
  String get stockStandardCost => 'Coste estándar';

  @override
  String get stockStateAvailable => 'En stock';

  @override
  String get stockStateNotStocked => 'Sin gestión de stock';

  @override
  String get stockStatePartial => 'Parcialmente en stock';

  @override
  String get stockStateReserved => 'Reservado';

  @override
  String get stockStateShort => 'Sin stock';

  @override
  String get stockTitle => 'Existencias';

  @override
  String get stockTo => 'Hacia';

  @override
  String get stockTrackingLot => 'lote';

  @override
  String get stockTransfer => 'Traspasar';

  @override
  String get stockWarehouse => 'Almacén';

  @override
  String get stockWarehouses => 'Almacenes';

  @override
  String get street => 'Calle';

  @override
  String get streetAddress => 'Calle y número';

  @override
  String structureRevision(int revision) {
    return 'Estructura (revisión $revision)';
  }

  @override
  String get subassembly => 'subconjunto';

  @override
  String get subject => 'Asunto';

  @override
  String get subjectContains => 'El asunto contiene';

  @override
  String get subjectOptional => 'Asunto (opcional)';

  @override
  String suggested(String value) {
    return 'Sugerencia: $value';
  }

  @override
  String get tag => 'Etiqueta';

  @override
  String get suspendContract => 'Suspender';

  @override
  String get tags => 'Etiquetas';

  @override
  String get takePhoto => 'Hacer una foto';

  @override
  String get tasks => 'Tareas';

  @override
  String tasksProgress(String done, String total) {
    return '$done de $total hechas';
  }

  @override
  String get team => 'Equipo';

  @override
  String get teamLead => 'Responsable';

  @override
  String get teamMembers => 'Miembros';

  @override
  String get teamName => 'Nombre del equipo';

  @override
  String get teams => 'Equipos';

  @override
  String get technicians => 'Técnicos';

  @override
  String get text => 'Texto';

  @override
  String get ticket => 'Ticket';

  @override
  String get ticketClosed => 'Cerrado';

  @override
  String get ticketDetails => 'Detalles del ticket';

  @override
  String get ticketInProgress => 'En tramitación';

  @override
  String get ticketMerged => 'Tickets combinados';

  @override
  String get ticketOpened => 'Ticket creado';

  @override
  String ticketSplit(String number) {
    return 'Ticket $number creado';
  }

  @override
  String get ticketStatusClosed => 'cerrado';

  @override
  String get ticketStatusNew => 'nuevo';

  @override
  String get ticketStatusOpen => 'abierto';

  @override
  String get ticketStatusPendingCustomer => 'esperando al cliente';

  @override
  String get ticketStatusPendingInternal => 'esperando a un compañero';

  @override
  String get ticketVolume => 'Tickets creados y cerrados';

  @override
  String get ticketWaitingForYou => 'Esperando su respuesta';

  @override
  String get tickets => 'Tickets';

  @override
  String get timeLogged => 'Tiempo registrado';

  @override
  String get timeNote => 'Nota';

  @override
  String timeTotal(String total, String billable) {
    return 'Total $total, facturable $billable';
  }

  @override
  String timeTotals(String work, String travel) {
    return 'Trabajo $work · Desplazamiento $travel';
  }

  @override
  String get timeTracking => 'Registro de tiempo';

  @override
  String get timeZone => 'Zona horaria';

  @override
  String get title => 'Título';

  @override
  String get titleCustomerValueRequired =>
      'El título, el cliente y un valor válido son obligatorios.';

  @override
  String get titleMr => 'Sr.';

  @override
  String get titleMs => 'Sra.';

  @override
  String get titleNone => 'No especificado';

  @override
  String get to => 'Para';

  @override
  String get topCategories => 'Categorías principales';

  @override
  String get topTags => 'Etiquetas principales';

  @override
  String get total => 'Total';

  @override
  String productsTotalChange(String from, String to, String difference) {
    return 'Productos sin envío ni IVA: $from → $to ($difference)';
  }

  @override
  String get totalExclVat => 'Total sin IVA';

  @override
  String get totalInclVat => 'Total IVA incl.';

  @override
  String get totalsToBuyOrMake => 'Totales a comprar o fabricar';

  @override
  String get tracking => 'Trazabilidad';

  @override
  String get twoLetters => '2 letras';

  @override
  String get type => 'Tipo';

  @override
  String get unassigned => 'Sin asignar';

  @override
  String get unchanged => 'Sin cambios';

  @override
  String get unit => 'Unidad';

  @override
  String get unitPrice => 'Precio unitario';

  @override
  String get unitPriceExclVat => 'Precio unitario (sin IVA)';

  @override
  String get unknownCaller => 'Llamante desconocido';

  @override
  String get until => 'Hasta';

  @override
  String get upcomingVisits => 'Próximas visitas';

  @override
  String get updateToCurrentPrices => 'Actualizar a precios actuales';

  @override
  String get updated => 'Actualizado';

  @override
  String get uploadingFile => 'Subiendo…';

  @override
  String get uploadingPhotos => 'Subiendo…';

  @override
  String get useAnotherPriceListDate => 'Usar otra fecha de tarifa';

  @override
  String get validFrom => 'Válido desde';

  @override
  String validFromDate(String date) {
    return 'Válido desde $date';
  }

  @override
  String validUntil(String date) {
    return 'válido hasta $date';
  }

  @override
  String get value => 'Valor';

  @override
  String get vat => 'IVA';

  @override
  String get vatNumber => 'NIF-IVA';

  @override
  String vatOn(String rate, String base) {
    return 'IVA $rate % sobre $base';
  }

  @override
  String get vatPercent => 'IVA %';

  @override
  String get viaPortal => 'A través del portal de clientes';

  @override
  String get visitTemplates => 'Visitas';

  @override
  String get visitTitleRequired => 'Cada visita necesita un título';

  @override
  String get warranty => 'Garantía';

  @override
  String get warrantyExpired => 'Garantía vencida';

  @override
  String get warrantyIn => 'En garantía';

  @override
  String get warrantyOut => 'Fuera de garantía';

  @override
  String get warrantyUnknown => 'Desconocida';

  @override
  String get weekdayFriday => 'Viernes';

  @override
  String get weekdayMonday => 'Lunes';

  @override
  String get weekdaySaturday => 'Sábado';

  @override
  String get weekdaySunday => 'Domingo';

  @override
  String get weekdayThursday => 'Jueves';

  @override
  String get weekdayTuesday => 'Martes';

  @override
  String get weekdayWednesday => 'Miércoles';

  @override
  String warrantyUntilDate(String date) {
    return 'hasta $date';
  }

  @override
  String get warrantyUntilLabel => 'Garantía hasta';

  @override
  String weighted(String amount) {
    return 'ponderado $amount';
  }

  @override
  String get whatWasAsked => '¿Qué se ha preguntado?';

  @override
  String get whoIsCalling => '¿Quién llama?';

  @override
  String get won => 'Ganada';

  @override
  String get workFrom => 'Desde';

  @override
  String get workKindTravel => 'Desplazamiento';

  @override
  String get workKindWork => 'Trabajo';

  @override
  String get workLogs => 'Tiempo registrado';

  @override
  String get workUntil => 'Hasta';

  @override
  String writeIn(String language) {
    return 'Escribir en $language';
  }

  @override
  String get yes => 'Sí';

  @override
  String get yourMessage => 'Su mensaje';

  @override
  String get yourReference => 'Su referencia (RMA o pedido)';

  @override
  String get zone => 'Zona';

  @override
  String zoneAndRebate(String zone, String percent) {
    return 'Zona $zone · descuento general $percent %';
  }

  @override
  String get zonesNoneAll => 'Zonas (sin selección = todas)';

  @override
  String get kbTitle => 'Base de conocimiento';

  @override
  String get kbNewArticle => 'Nuevo artículo';

  @override
  String get kbSearchHint => 'Buscar artículos';

  @override
  String get kbAllCategories => 'Todas las categorías';

  @override
  String get kbAllStatuses => 'Todos los estados';

  @override
  String get kbAllAudiences => 'Todos los públicos';

  @override
  String get kbManageCategories => 'Categorías';

  @override
  String get kbStatusPublished => 'Publicado';

  @override
  String get kbStatusArchived => 'Archivado';

  @override
  String get kbAudience => 'Público';

  @override
  String get kbAudienceInternal => 'Interno (solo personal)';

  @override
  String get kbAudiencePortal => 'Portal de clientes';

  @override
  String get kbAudiencePublic => 'Público';

  @override
  String get kbArticleTitle => 'Título';

  @override
  String get kbSummary => 'Resumen';

  @override
  String get kbBody => 'Texto (Markdown)';

  @override
  String get kbPreview => 'Vista previa';

  @override
  String get kbWriteTab => 'Escribir';

  @override
  String get kbMarkdownHelp =>
      'Use # para títulos, **negrita**, *cursiva*, - para listas, [texto](https://…) para enlaces y ![descripción](image:ID) para imágenes.';

  @override
  String get kbTagsLabel => 'Etiquetas (separadas por comas)';

  @override
  String get kbRelated => 'Artículos relacionados';

  @override
  String get kbAddRelated => 'Añadir artículo relacionado';

  @override
  String get kbImages => 'Imágenes';

  @override
  String get kbUploadImage => 'Subir imagen';

  @override
  String get kbInsertImage => 'Insertar en el texto';

  @override
  String get kbPublish => 'Publicar';

  @override
  String get kbUnpublish => 'Despublicar';

  @override
  String get kbArchive => 'Archivar';

  @override
  String get kbHistory => 'Historial de versiones';

  @override
  String kbVersionLine(int version, String name, String date) {
    return 'Versión $version · $name · $date';
  }

  @override
  String get kbRestore => 'Restaurar esta versión';

  @override
  String kbRestoredFrom(int version) {
    return 'restaurada desde la versión $version';
  }

  @override
  String kbFeedback(int yes, int no) {
    return '$yes lo encontraron útil, $no no';
  }

  @override
  String kbViews(int count) {
    return '$count vistas';
  }

  @override
  String get kbNoArticles => 'No se encontraron artículos.';

  @override
  String get kbNoCategory => 'Sin categoría';

  @override
  String get kbMainCategory => 'Categoría principal';

  @override
  String get kbSortOrder => 'Orden';

  @override
  String get kbNewCategory => 'Nueva categoría';

  @override
  String get kbCategoryNames => 'Nombre por idioma';

  @override
  String get kbDelete => 'Eliminar';

  @override
  String get kbSuggestedArticles => 'Artículos sugeridos';

  @override
  String get kbNoSuggestions => 'No hay artículos que coincidan.';

  @override
  String get kbInsertText => 'Insertar texto';

  @override
  String get kbInsertLink => 'Insertar enlace';

  @override
  String get kbInsertArticle => 'Insertar artículo';

  @override
  String get kbNoPortalLink =>
      'Este artículo es interno: solo se puede insertar su texto.';

  @override
  String get kbHelp => 'Ayuda';

  @override
  String get kbHelpHeading => '¿Cómo podemos ayudarle?';

  @override
  String get kbHelpSearch => 'Buscar en los artículos de ayuda';

  @override
  String get kbHelpEmpty =>
      'No se encontraron artículos. Puede abrir un ticket.';

  @override
  String get kbWasHelpful => '¿Le ha resultado útil este artículo?';

  @override
  String get kbYes => 'Sí';

  @override
  String get kbNo => 'No';

  @override
  String get kbThanks => 'Gracias por sus comentarios.';

  @override
  String get kbStillNeedHelp => '¿Necesita más ayuda? Abra un ticket.';

  @override
  String get kbRelatedReading => 'Artículos relacionados';

  @override
  String get kbDeflectionTitle =>
      'Estos artículos quizá ya respondan a su pregunta';

  @override
  String kbShownIn(String language) {
    return 'No disponible en su idioma; se muestra en $language.';
  }

  @override
  String kbUpdatedOn(String date) {
    return 'Actualizado el $date';
  }

  @override
  String offlineWaiting(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count cambios pendientes de sincronizar',
      one: '1 cambio pendiente de sincronizar',
    );
    return '$_temp0';
  }

  @override
  String offlineFailedCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count requieren su atención',
      one: '1 requiere su atención',
    );
    return '$_temp0';
  }

  @override
  String get offlineQueueTitle => 'Cambios pendientes de sincronizar';

  @override
  String get offlineSyncNow => 'Sincronizar ahora';

  @override
  String get offlineRetry => 'Reintentar';

  @override
  String get offlineDiscard => 'Descartar';

  @override
  String get offlineDiscardQuestion =>
      '¿Descartar este cambio? No se enviará al servidor.';

  @override
  String get offlineAllSynced => 'Todo está sincronizado.';

  @override
  String offlineAttempts(int count) {
    return 'Intentos: $count';
  }

  @override
  String offlineLastError(String message) {
    return 'Último error: $message';
  }

  @override
  String get offlineRejected =>
      'El servidor rechazó este cambio. Puede que el trabajo se haya reasignado, cancelado o modificado entretanto.';

  @override
  String offlineDataFrom(String time) {
    return 'Sin conexión: datos de $time';
  }

  @override
  String get offlinePendingSync => 'Pendiente de sincronizar';

  @override
  String get offlineQueuedMessage =>
      'Sin conexión: guardado en este dispositivo, se enviará cuando vuelva a estar en línea.';

  @override
  String offlinePhotosWaiting(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count fotos pendientes de subir',
      one: '1 foto pendiente de subir',
    );
    return '$_temp0';
  }

  @override
  String get offlineTooLarge =>
      'Demasiado grande para guardarlo sin conexión. Conéctese para enviarlo.';

  @override
  String get offlineReload => 'Recargar';

  @override
  String get offlineOpStart => 'Iniciar el trabajo';

  @override
  String get offlineOpTaskDone => 'Tarea marcada';

  @override
  String get offlineOpTaskUndone => 'Tarea desmarcada';

  @override
  String get offlineOpTaskAdd => 'Nueva tarea';

  @override
  String get offlineOpWorkLog => 'Registro de tiempo';

  @override
  String get offlineOpPart => 'Pieza utilizada';

  @override
  String get offlineOpComplete => 'Completar el trabajo';

  @override
  String get offlineOpDecline => 'El cliente se negó a firmar';

  @override
  String get offlineOpSignature => 'Firma del cliente';

  @override
  String get offlineOpPhoto => 'Foto';

  @override
  String get collabEscalate => 'Escalar';

  @override
  String get collabEscalateTitle => 'Escalar ticket';

  @override
  String get collabDelegate => 'Transferir';

  @override
  String get collabDelegateTitle => 'Transferir ticket';

  @override
  String get collabEscalateTo => 'Escalar a';

  @override
  String get collabNoTarget => 'Responsables del equipo del ticket';

  @override
  String get collabPerson => 'Compañero';

  @override
  String get collabTeam => 'Equipo';

  @override
  String get collabHandOverTo => 'Nuevo responsable';

  @override
  String get collabReasonRequired => 'Motivo (obligatorio)';

  @override
  String get collabRaisePriority => 'Subir la prioridad un nivel';

  @override
  String get collabCommentOptional =>
      'Comentario para el nuevo responsable (opcional)';

  @override
  String get collabInternalHint => 'Interno: nunca se envía al cliente.';

  @override
  String get collabEscalated => 'Ticket escalado';

  @override
  String get collabHandedOver => 'Ticket transferido';

  @override
  String get collabEscalatedBadge => 'Escalado';

  @override
  String collabEscalatedBy(String name) {
    return 'Escalado por $name';
  }

  @override
  String get collabWatch => 'Seguir';

  @override
  String get collabUnwatch => 'Dejar de seguir';

  @override
  String get collabWatchers => 'Seguidores';

  @override
  String get collabNoWatchers => 'Nadie sigue este ticket';

  @override
  String get collabAddWatcher => 'Añadir seguidor';

  @override
  String get collabActivity => 'Actividad';

  @override
  String get collabNoActivity => 'Aún no hay actividad';

  @override
  String collabSeenBy(String readers) {
    return 'Visto por $readers';
  }

  @override
  String get collabFilterUnread => 'No leídos';

  @override
  String get collabFilterWatching => 'Seguidos';

  @override
  String get collabFilterEscalated => 'Escalados';

  @override
  String get collabUnreadTooltip => 'No leído';

  @override
  String collabReplying(int count, String names) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$names están respondiendo',
      one: '$names está respondiendo',
    );
    return '$_temp0 — coordínese antes de enviar';
  }

  @override
  String collabViewing(int count, String names) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$names están viendo este ticket',
      one: '$names está viendo este ticket',
    );
    return '$_temp0';
  }

  @override
  String collabConfirmSend(int count, String names) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$names están respondiendo a este ticket ahora mismo.',
      one: '$names está respondiendo a este ticket ahora mismo.',
    );
    return '$_temp0 ¿Enviar de todos modos?';
  }

  @override
  String get collabSendAnyway => 'Enviar de todos modos';

  @override
  String get collabSystem => 'Sistema';

  @override
  String collabEventCreated(String actor) {
    return '$actor creó el ticket';
  }

  @override
  String collabEventAssignedOwner(String actor, String owner) {
    return '$actor asignó el ticket a $owner';
  }

  @override
  String collabEventOwnerCleared(String actor) {
    return '$actor quitó al responsable';
  }

  @override
  String collabEventTeam(String actor, String team) {
    return '$actor movió el ticket al equipo $team';
  }

  @override
  String collabEventTeamCleared(String actor) {
    return '$actor quitó el equipo';
  }

  @override
  String collabEventDelegated(String actor, String owner) {
    return '$actor transfirió el ticket a $owner';
  }

  @override
  String collabEventEscalated(String actor) {
    return '$actor escaló el ticket';
  }

  @override
  String collabEventStatus(String actor, String before, String after) {
    return '$actor cambió el estado de $before a $after';
  }

  @override
  String collabEventPriority(String actor, String before, String after) {
    return '$actor cambió la prioridad de $before a $after';
  }

  @override
  String collabEventMergedInto(String actor, String number) {
    return '$actor fusionó este ticket en $number';
  }

  @override
  String collabEventMergedFrom(String actor, String number) {
    return '$actor fusionó $number en este ticket';
  }

  @override
  String collabEventTagAdded(String actor, String tag) {
    return '$actor añadió la etiqueta $tag';
  }

  @override
  String collabEventTagRemoved(String actor, String tag) {
    return '$actor quitó la etiqueta $tag';
  }

  @override
  String collabMsgCustomer(String actor) {
    return '$actor (cliente) escribió';
  }

  @override
  String collabMsgReply(String actor) {
    return '$actor respondió al cliente';
  }

  @override
  String collabMsgNote(String actor) {
    return '$actor añadió una nota interna';
  }

  @override
  String collabMsgCall(String actor) {
    return '$actor registró una llamada';
  }

  @override
  String get planningTitle => 'Planificación';

  @override
  String get planningDay => 'Día';

  @override
  String get planningWeek => 'Semana';

  @override
  String get planningToday => 'Hoy';

  @override
  String get planningPrevious => 'Anterior';

  @override
  String get planningNext => 'Siguiente';

  @override
  String get planningBacklog => 'Por planificar';

  @override
  String get planningBacklogEmpty => 'Nada más que planificar.';

  @override
  String get planningSkillFilter => 'Habilidad';

  @override
  String get planningAllSkills => 'Todas las habilidades';

  @override
  String get planningNoTechnicians =>
      'No hay técnicos que mostrar. Asigne el rol de técnico a los usuarios.';

  @override
  String get planningPlan => 'Planificar';

  @override
  String planningScheduleJob(String number) {
    return 'Planificar $number';
  }

  @override
  String get planningSuggestions => 'Sugerencias';

  @override
  String get planningShowSuggestions => 'Mostrar sugerencias';

  @override
  String get planningNoSuggestions =>
      'No se encontró ningún hueco libre en las próximas dos semanas.';

  @override
  String planningMissingSkills(int count) {
    return 'Faltan $count habilidad(es)';
  }

  @override
  String get planningRequiredSkills => 'Habilidades requeridas';

  @override
  String get planningChecking => 'Comprobando…';

  @override
  String get planningNoConflicts => 'Sin conflictos.';

  @override
  String get planningConfirmTitle => '¿Planificar con conflictos?';

  @override
  String get planningConfirmBody => 'Esta planificación tiene errores:';

  @override
  String get planningScheduleAnyway => 'Planificar de todos modos';

  @override
  String get planningScheduled => 'Trabajo planificado.';

  @override
  String planningConflictDoubleBooked(String name) {
    return '$name tiene otro trabajo en ese momento (incluido el desplazamiento).';
  }

  @override
  String planningConflictOutsideHours(String name) {
    return '$name queda fuera del horario de trabajo.';
  }

  @override
  String planningConflictOnAbsence(String name) {
    return '$name está ausente.';
  }

  @override
  String planningConflictMissingSkill(String name, String skill) {
    return '$name no tiene la habilidad $skill.';
  }

  @override
  String get planningConflictCrossesDays =>
      'El trabajo se extiende pasada la medianoche.';

  @override
  String get planningAbsenceVacation => 'Vacaciones';

  @override
  String get planningAbsenceSick => 'Enfermedad';

  @override
  String get planningAbsenceTraining => 'Formación';

  @override
  String get planningAbsenceOther => 'Otra ausencia';

  @override
  String get planningAgendaEmpty => 'Nada planificado.';

  @override
  String get planningSettingsTitle => 'Ajustes de planificación';

  @override
  String get planningAbsences => 'Ausencias';

  @override
  String get planningHours => 'Horario de trabajo';

  @override
  String get planningSkills => 'Habilidades';

  @override
  String get planningGeneral => 'General';

  @override
  String get planningAddAbsence => 'Añadir ausencia';

  @override
  String get planningNoAbsences => 'No hay ausencias registradas.';

  @override
  String get planningTechnician => 'Técnico';

  @override
  String get planningKind => 'Tipo';

  @override
  String get planningDefaultHours => 'Predeterminado de la empresa';

  @override
  String get planningUsesDefault =>
      'Sin horario propio: se aplica el predeterminado de la empresa.';

  @override
  String get planningAddBlock => 'Añadir tramo';

  @override
  String get planningAddSkill => 'Añadir habilidad';

  @override
  String get planningTechnicianSkills => 'Habilidades por técnico';

  @override
  String planningSkillLevel(int n) {
    return 'Nivel $n';
  }

  @override
  String get planningTravelBuffer =>
      'Tiempo de desplazamiento entre trabajos (minutos)';

  @override
  String get planningTravelBufferHelp =>
      'Se reserva entre dos trabajos de un técnico el mismo día.';

  @override
  String get planningDefaultJobMinutes =>
      'Duración predeterminada de un trabajo (minutos)';

  @override
  String get planningTimezone => 'Zona horaria';

  @override
  String get adminTitle => 'Administración';

  @override
  String get adminUsers => 'Usuarios';

  @override
  String get adminModules => 'Módulos';

  @override
  String get newUser => 'Nuevo usuario';

  @override
  String get editUser => 'Editar usuario';

  @override
  String get userDisabledModules => 'Módulos desactivados';

  @override
  String get userNotSignedIn => 'Aún no ha iniciado sesión';

  @override
  String get userResendInvitation => 'Enviar la invitación de nuevo';

  @override
  String get userInvitationSent => 'Invitación enviada';

  @override
  String get userCreated => 'Usuario añadido';

  @override
  String get userFilterAll => 'Todos los miembros';

  @override
  String get userFilterActive => 'Miembros activos';

  @override
  String get userFilterInactive => 'Miembros desactivados';

  @override
  String get userActiveHelp =>
      'Un miembro desactivado ya no puede iniciar sesión en esta empresa.';

  @override
  String get userCannotDeactivateSelf => 'No puede desactivarse a sí mismo.';

  @override
  String get userCannotDropOwnAdmin =>
      'No puede quitarse su propio rol de administrador.';

  @override
  String userRolePermissions(int count) {
    return '$count permisos';
  }

  @override
  String get userModulesHelp =>
      'Desactive módulos para esta persona. Los módulos desactivados por la empresa no se pueden volver a activar aquí.';

  @override
  String get userModuleCompanyOff => 'Desactivado para toda la empresa';

  @override
  String moduleRequires(String modules) {
    return 'Requiere $modules';
  }

  @override
  String get modulesIntro =>
      'Desactive los módulos que esta empresa no usa. Dejan de verse y el servidor rechaza sus peticiones.';

  @override
  String get modulesSaved => 'Módulos guardados';

  @override
  String get userRoleAdmin => 'Administrador';

  @override
  String get userRoleSalesManager => 'Responsable de ventas';

  @override
  String get userRoleSales => 'Ventas';

  @override
  String get userRoleSupport => 'Soporte';

  @override
  String get userRoleProduction => 'Producción';

  @override
  String get userRoleWarehouse => 'Almacén';

  @override
  String get userRoleProductManager => 'Gestor de productos';

  @override
  String get userRoleWorkshop => 'Taller';

  @override
  String get userRoleTechnician => 'Técnico';

  @override
  String get userRoleDispatcher => 'Planificador';

  @override
  String get userRoleLogistics => 'Logística';

  @override
  String get userRoleViewer => 'Lector';

  @override
  String get moduleCustomers => 'Clientes';

  @override
  String get moduleProducts => 'Productos';

  @override
  String get moduleSales => 'Ventas';

  @override
  String get moduleEngineering => 'Ingeniería';

  @override
  String get moduleInventory => 'Inventario';

  @override
  String get moduleHelpdesk => 'Helpdesk';

  @override
  String get moduleKb => 'Base de conocimiento';

  @override
  String get moduleRepairs => 'Reparaciones';

  @override
  String get moduleField => 'Servicio de campo';

  @override
  String get purpose => 'Finalidad';

  @override
  String get purposeInstallation => 'Instalación';

  @override
  String get purposeStock => 'Stock';

  @override
  String get deliveryCountry => 'País de entrega';

  @override
  String deliveryCountryDefault(String country) {
    return 'Por defecto: $country';
  }

  @override
  String get deliveryCountryInvalid =>
      'Introduzca un código de país de dos letras, por ejemplo BE.';

  @override
  String get shippingNotRequired => 'no necesario';

  @override
  String get shippingPending => 'pendiente';

  @override
  String get shippingQuoted => 'cotizado';

  @override
  String get shippingRequested => 'solicitado';

  @override
  String get shippingPendingBanner =>
      'A la espera de un presupuesto de envío de logística. La oferta no se puede enviar hasta que se introduzca el coste.';

  @override
  String get shippingQuotedBanner =>
      'El coste de envío está cotizado y figura en la oferta como línea de envío.';

  @override
  String get shippingLineNote => 'Añadido por logística';

  @override
  String get sendAnyway => 'Enviar de todos modos';

  @override
  String sendAnywayHelp(String message) {
    return '$message\n\nComo logística puede enviar la oferta sin el coste de envío. ¿Enviarla de todos modos?';
  }

  @override
  String get shippingRequests => 'Solicitudes de envío';

  @override
  String get shippingQueue => 'Abiertas y solicitadas';

  @override
  String get markAsRequested => 'Marcar como solicitado';

  @override
  String get enterQuote => 'Introducir precio';

  @override
  String get carrier => 'Transportista';

  @override
  String get amount => 'Importe';

  @override
  String get currency => 'Moneda';

  @override
  String get quotedAmount => 'Importe cotizado';

  @override
  String get requestedAt => 'Solicitado el';

  @override
  String get amountRequired => 'Introduzca un importe válido.';

  @override
  String get carrierRequired => 'Introduzca el transportista.';

  @override
  String get packagingCategory => 'Categoría de embalaje';

  @override
  String get packagingCategories => 'Categorías de embalaje';

  @override
  String get newPackagingCategory => 'Nueva categoría de embalaje';

  @override
  String get requiresShippingQuote => 'Requiere un presupuesto de envío';

  @override
  String get lengthMm => 'Largo (mm)';

  @override
  String get widthMm => 'Ancho (mm)';

  @override
  String get heightMm => 'Alto (mm)';

  @override
  String get weightG => 'Peso (g)';

  @override
  String get dimensions => 'Dimensiones';

  @override
  String dimensionsValue(int length, int width, int height) {
    return '$length x $width x $height mm';
  }

  @override
  String get weight => 'Peso';

  @override
  String weightValue(int grams) {
    return '$grams g';
  }

  @override
  String get positiveWholeNumbers =>
      'Las dimensiones y el peso deben ser números enteros positivos o estar vacíos.';

  @override
  String get powerTab => 'Alimentación';

  @override
  String get powerNotElectrical =>
      'No hay alimentaciones indicadas: no es un producto eléctrico y nunca se compara con una red eléctrica.';

  @override
  String get powerAddSupply => 'Añadir alimentación';

  @override
  String get currentType => 'Tipo de corriente';

  @override
  String get currentAc => 'CA';

  @override
  String get currentDc => 'CC';

  @override
  String get voltageV => 'Tensión (V)';

  @override
  String get phases => 'Fases';

  @override
  String get frequencyHz => 'Frecuencia (Hz)';

  @override
  String get frequencyBoth => '50 y 60 Hz';

  @override
  String get tolerancePct => 'Tolerancia (%)';

  @override
  String get powerVoltageInvalid =>
      'Introduzca la tensión como un número entero positivo.';

  @override
  String get powerToleranceInvalid =>
      'La tolerancia debe ser un número entero de 0 a 50.';

  @override
  String get powerSupplyDuplicate => 'La misma alimentación figura dos veces.';

  @override
  String get powerCheckTitle => 'Comprobación de compatibilidad con la red';

  @override
  String get powerCheckHelp =>
      'Compara las alimentaciones guardadas con las redes eléctricas de un país.';

  @override
  String get powerGrid => 'Red eléctrica';

  @override
  String get powerAnyGrid => 'Cualquier red de este país';

  @override
  String get powerCheck => 'Comprobar';

  @override
  String powerResultCompatible(String country) {
    return 'Compatible con la red eléctrica de $country.';
  }

  @override
  String powerResultIncompatible(
    String country,
    String supported,
    String available,
  ) {
    return 'No encaja en la red de $country. Admitido: $supported. Disponible allí: $available.';
  }

  @override
  String get powerResultExternal =>
      'Funciona solo con corriente continua y necesita una fuente externa; no se compara con la red.';

  @override
  String get powerResultNotApplicable =>
      'No es un producto eléctrico; no hay nada que comprobar.';

  @override
  String powerResultUnknown(String country) {
    return 'No hay datos de red para $country: no se puede comprobar la compatibilidad.';
  }

  @override
  String get powerStatusCompatible => 'Compatible';

  @override
  String get powerStatusIncompatible => 'Incompatible';

  @override
  String get powerStatusExternal => 'Necesita fuente externa';

  @override
  String get powerStatusNotApplicable => 'No aplicable';

  @override
  String get powerStatusUnknown => 'Red desconocida';

  @override
  String get relatedTab => 'Relacionados';

  @override
  String get relationRules => 'Reglas';

  @override
  String get relationSuggestions => 'Sugerencias';

  @override
  String get relationNoRules => 'No se aplica ninguna regla a este producto.';

  @override
  String get relationNoSuggestions => 'Aún no hay sugerencias.';

  @override
  String get relationKindCompatible => 'Compatible';

  @override
  String get relationKindIncompatible => 'Incompatible';

  @override
  String get relationKindSuggested => 'Sugerido';

  @override
  String get relationAddRule => 'Añadir regla';

  @override
  String get relationEdit => 'Editar relación';

  @override
  String get relationsTitle => 'Relaciones entre productos';

  @override
  String get newRelation => 'Nueva relación';

  @override
  String get relationKindLabel => 'Tipo';

  @override
  String get relationSideA => 'Primer lado';

  @override
  String get relationSideB => 'Segundo lado';

  @override
  String get relationBidirectional => 'Se aplica en ambos sentidos';

  @override
  String get relationStrength => 'Fuerza (1-100)';

  @override
  String get relationStrengthHelp => 'Solo la usan las reglas sugeridas.';

  @override
  String get relationNeedSides =>
      'Elija un producto o una familia en ambos lados.';

  @override
  String get relationStrengthInvalid =>
      'La fuerza debe ser un número entero de 1 a 100.';

  @override
  String get relationsEmpty => 'Sin relaciones.';

  @override
  String relationFamilyValue(String code) {
    return 'Familia $code';
  }

  @override
  String get suggestionReasonRule => 'Regla';

  @override
  String get suggestionReasonOften => 'Se compran juntos a menudo';

  @override
  String suggestionStrength(int n) {
    return 'Fuerza $n';
  }

  @override
  String suggestionConfidence(int pct) {
    return 'Confianza $pct %';
  }

  @override
  String suggestionBecause(String code) {
    return 'Por $code';
  }

  @override
  String get powerGridsTitle => 'Redes eléctricas';

  @override
  String get newPowerGrid => 'Nueva red eléctrica';

  @override
  String get powerGridInvalid =>
      'Introduzca un código de país de dos letras, una tensión, 1 o 3 fases y 50 o 60 Hz.';

  @override
  String get warningKindPowerGrid => 'Red eléctrica';

  @override
  String get warningKindIncompatible => 'Productos incompatibles';

  @override
  String hintsPowerIncompatible(String code) {
    return '$code no encaja en la red eléctrica del lugar de entrega.';
  }

  @override
  String hintsSupported(String value) {
    return 'Admitido: $value';
  }

  @override
  String hintsAvailable(String value) {
    return 'Disponible: $value';
  }

  @override
  String hintsPowerExternal(String code) {
    return '$code funciona con corriente continua y necesita una fuente externa.';
  }

  @override
  String hintsPowerUnknown(String country, String code) {
    return 'No hay datos de red para $country, así que no se pudo comprobar $code. Añada la red del país en Redes eléctricas, en la página Productos.';
  }

  @override
  String hintsIncompatibleProducts(String a, String b) {
    return '$a y $b no son compatibles.';
  }

  @override
  String get suggestionsTitle => 'Adiciones sugeridas';

  @override
  String get hintsTitle => 'Advertencias y sugerencias';

  @override
  String revisionShippingExisting(String amount) {
    return 'Gastos de envío existentes: $amount';
  }

  @override
  String get revisionShippingReset =>
      'Con los precios actuales, estos gastos de envío se restablecerán. Se necesita un nuevo presupuesto de envío de logística antes de poder enviar el presupuesto revisado.';

  @override
  String revisionShippingKept(String amount) {
    return 'Con los precios acordados, el envío presupuestado se mantiene en el presupuesto revisado (gastos actuales: $amount).';
  }

  @override
  String get packagesReceiving => 'Recepción de mercancías';

  @override
  String get packagesIntake => 'Recepción de reparaciones';

  @override
  String get registerParcel => 'Registrar paquete';

  @override
  String get registeredParcels => 'Paquetes registrados';

  @override
  String get noParcels => 'Ningún paquete.';

  @override
  String get sender => 'Remitente';

  @override
  String get trackingNumber => 'Número de seguimiento';

  @override
  String get parcelCount => 'Número de bultos';

  @override
  String get packageCondition => 'Estado de la caja';

  @override
  String get conditionIntact => 'Intacto';

  @override
  String get conditionDamaged => 'Dañado';

  @override
  String get conditionOpened => 'Abierto';

  @override
  String get conditionWet => 'Mojado';

  @override
  String get conditionNote => 'Nota sobre el estado';

  @override
  String get removePhoto => 'Quitar foto';

  @override
  String parcelRegistered(String number) {
    return 'Paquete $number registrado';
  }

  @override
  String get pkgStatusRegistered => 'Registrado';

  @override
  String get pkgStatusOpened => 'Abierto';

  @override
  String get pkgStatusLinked => 'Asociado a una reparación';

  @override
  String get pkgStatusClosed => 'Cerrado';

  @override
  String get packageKindRepair => 'Reparación';

  @override
  String get packageKindOther => 'Otro (sin reparación)';

  @override
  String get packageKindReturn => 'Devolver al remitente';

  @override
  String get toOpen => 'Por abrir';

  @override
  String get toProcess => 'Por procesar';

  @override
  String get toRoute => 'Por derivar';

  @override
  String get nothingHere => 'Nada que hacer aquí.';

  @override
  String get openParcel => 'Abrir paquete';

  @override
  String get parcelContents => '¿Qué contiene el paquete?';

  @override
  String get closeParcel => 'Cerrar paquete';

  @override
  String get noteRequired => 'La nota es obligatoria.';

  @override
  String get linkToRepair => 'Asociar a una reparación';

  @override
  String get repairSuggestions => 'Reparaciones sugeridas';

  @override
  String get noSuggestions => 'Ninguna reparación anunciada coincide.';

  @override
  String suggestedCustomer(String name) {
    return 'Cliente sugerido: $name';
  }

  @override
  String suggestionScore(String score) {
    return 'Puntuación $score';
  }

  @override
  String get reasonSerial => 'Mismo número de serie';

  @override
  String get reasonCustomer => 'Mismo cliente';

  @override
  String get reasonReference => 'Misma referencia';

  @override
  String get reasonTicket => 'Mismo ticket';

  @override
  String get reasonProduct => 'Producto similar';

  @override
  String get reasonRecent => 'Anunciado recientemente';

  @override
  String get confirmLinkTitle => 'Confirmar asociación';

  @override
  String confirmLinkBody(String parcel, String repair) {
    return 'El paquete $parcel contiene la reparación $repair. Una reparación anunciada se marcará como recibida.';
  }

  @override
  String get customerMismatchWarning =>
      'El cliente del paquete no es el de la reparación.';

  @override
  String get confirmCustomerMismatch =>
      'Sí, lo envió otra persona (por ejemplo un compañero)';

  @override
  String get confirmLink => 'Confirmar y asociar';

  @override
  String get linkConflict =>
      'La reparación ha cambiado mientras tanto. La lista se ha actualizado; compruébela e inténtelo de nuevo.';

  @override
  String get createNewRepair => 'Crear nueva reparación';

  @override
  String get parcelsSection => 'Paquetes';

  @override
  String get noParcelsLinked => 'Ningún paquete asociado.';

  @override
  String get unlinkParcel => 'Desvincular paquete';

  @override
  String get parcelsToLink => 'Paquetes en espera de esta reparación';

  @override
  String get linkParcel => 'Asociar paquete';

  @override
  String get parcelLinked => 'Paquete asociado';

  @override
  String get parcelUnlinked => 'Paquete desvinculado';

  @override
  String get customerUnknown => 'Cliente desconocido';

  @override
  String get findingsSection => 'Hallazgos';

  @override
  String get findingsLabel => 'Lo que se ha constatado';

  @override
  String get diagnosisLabel => 'Diagnóstico';

  @override
  String get saveFindings => 'Guardar hallazgos';

  @override
  String get warrantySection => 'Garantía';

  @override
  String get warrantyVoid => 'Anulada (mal uso)';

  @override
  String get warrantyAutomatic => 'Resultado automático';

  @override
  String warrantyComparedWith(String date) {
    return 'Comparado con la fecha de llegada $date';
  }

  @override
  String get warrantyProvisional =>
      'Aún no recibido: comparado con hoy, por tanto provisional.';

  @override
  String serialUnitLabel(String unit) {
    return 'Unidad con número de serie: $unit';
  }

  @override
  String get noSerialUnit => 'Ninguna unidad con número de serie vinculada';

  @override
  String get linkSerialUnit => 'Vincular unidad con número de serie';

  @override
  String get unlinkSerialUnit => 'Desvincular unidad con número de serie';

  @override
  String get serialUnitSeveral =>
      'Varias unidades coinciden. Elija la correcta.';

  @override
  String get serialUnitNoResults => 'No se encontraron unidades';

  @override
  String get recheckWarranty => 'Volver a comprobar la garantía';

  @override
  String get markWarrantyVoid => 'Anular la garantía (mal uso)';

  @override
  String get removeWarrantyOverride => 'Quitar la anulación';

  @override
  String get warrantyVoidTitle => 'Anular la garantía';

  @override
  String get warrantyRemoveTitle => 'Quitar la anulación';

  @override
  String get warrantyReasonLabel => 'Motivo (obligatorio)';

  @override
  String warrantyOverriddenBy(String name, String date, String reason) {
    return 'Anulada por $name el $date: $reason';
  }

  @override
  String get eventFindings => 'Hallazgos actualizados';

  @override
  String get eventSerialUnit => 'Unidad con número de serie';

  @override
  String get eventWarrantyAuto => 'Garantía automática';

  @override
  String get eventWarrantyManual => 'Garantía fijada a mano';

  @override
  String get eventWarrantyVoid => 'Garantía anulada';

  @override
  String get eventWarrantyVoidRemoved => 'Anulación de garantía quitada';

  @override
  String get csvMenu => 'Exportar e importar CSV';

  @override
  String get csvExport => 'Exportar CSV';

  @override
  String get csvImport => 'Importar CSV';

  @override
  String csvExported(String name) {
    return '$name guardado';
  }

  @override
  String csvExportFailed(String error) {
    return 'Error al exportar: $error';
  }

  @override
  String csvImportTitle(String entity) {
    return 'Importar CSV: $entity';
  }

  @override
  String get csvImportHelp =>
      'Las filas se emparejan por código: un código existente se actualiza y un código nuevo se crea. Las columnas que faltan en el archivo no se modifican y nunca se elimina nada. Todo o nada: si alguna fila tiene un error, no se importa nada. Separado por punto y coma o coma, UTF-8.';

  @override
  String get csvChooseFile => 'Elegir archivo CSV';

  @override
  String get csvChooseOther => 'Elegir otro archivo';

  @override
  String get csvChecking => 'Comprobando el archivo...';

  @override
  String get csvImporting => 'Importando...';

  @override
  String get csvPreview => 'Vista previa: todavía no se ha importado nada';

  @override
  String get csvDone => 'Importación completada';

  @override
  String get csvToCreate => 'Por crear';

  @override
  String get csvToUpdate => 'Por actualizar';

  @override
  String get csvUnchanged => 'Sin cambios';

  @override
  String get csvCreated => 'Creados';

  @override
  String get csvUpdated => 'Actualizados';

  @override
  String get csvImportAction => 'Importar';

  @override
  String get csvEmptyFile => 'El archivo está vacío.';

  @override
  String get csvNotUtf8 =>
      'El archivo no es texto UTF-8. Guárdelo como CSV (UTF-8) e inténtelo de nuevo.';

  @override
  String get csvReadFailed => 'No se pudo leer el archivo.';

  @override
  String csvErrorsFound(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count problemas encontrados',
      one: '1 problema encontrado',
    );
    return '$_temp0. No se ha importado nada.';
  }

  @override
  String csvErrorRow(int row) {
    return 'Fila $row';
  }

  @override
  String csvErrorRowColumn(int row, String column) {
    return 'Fila $row, columna $column';
  }

  @override
  String csvAndMore(int count) {
    return 'y $count más';
  }

  @override
  String profileTooltip(String name) {
    return 'Perfil: $name';
  }

  @override
  String get tabTicket => 'Ticket';

  @override
  String get tabQuotation => 'Presupuesto';

  @override
  String get tabNewQuotation => 'Nuevo presupuesto';

  @override
  String get tabOrder => 'Pedido';

  @override
  String get tabRepair => 'Reparación';

  @override
  String get tabJob => 'Intervención';

  @override
  String get tabProduct => 'Producto';

  @override
  String get tabOpportunity => 'Oportunidad';

  @override
  String get tabStock => 'Existencias';

  @override
  String get tabArticle => 'Artículo';

  @override
  String get tabContract => 'Contrato';

  @override
  String get tabCount => 'Inventario';

  @override
  String get workspaceOverview => 'Resumen';

  @override
  String get openTabs => 'Pestañas abiertas';

  @override
  String get closeTab => 'Cerrar pestaña';

  @override
  String get openInNewTab => 'Abrir en una pestaña nueva';

  @override
  String get unsavedChangesTitle => 'Cambios sin guardar';

  @override
  String get unsavedChangesBody =>
      'Esta pestaña tiene cambios sin guardar. ¿Cerrarla y descartarlos?';

  @override
  String get discardChanges => 'Descartar cambios';

  @override
  String get remarkProduction => 'Nota de producción';

  @override
  String get remarkProductionHelp => 'Interna, nunca se muestra al cliente';

  @override
  String get remarkCustomer => 'Nota para el cliente';

  @override
  String get remarkCustomerHelp =>
      'Se imprime en la oferta y en la confirmación de pedido';

  @override
  String get remarkHistory => 'Notas anteriores';

  @override
  String get remarkHistoryThisCustomer => 'Este cliente';

  @override
  String get remarkHistoryThisLanguage => 'Este idioma';

  @override
  String get remarkHistoryAll => 'Todas';

  @override
  String get remarkHistoryEmpty =>
      'No hay notas anteriores para este producto.';

  @override
  String remarkHistoryUsed(int count, String quotation) {
    return 'Usada $count veces, la última en $quotation';
  }

  @override
  String get defaultProductionRemark => 'Nota de producción por defecto';

  @override
  String get optionsTab => 'Opciones';

  @override
  String get optionsIntro =>
      'Productos que se ofrecen junto con este en una oferta. Una opción obligatoria se añade automáticamente.';

  @override
  String get optionsNone => 'Este producto no tiene opciones.';

  @override
  String get optionAdd => 'Añadir opción';

  @override
  String get optionRequired => 'Obligatoria';

  @override
  String get optionQuantity => 'Cantidad por unidad';

  @override
  String get optionRemove => 'Quitar opción';

  @override
  String get optionsSaved => 'Opciones guardadas.';

  @override
  String get optionsInvalidQuantity =>
      'Introduzca una cantidad mayor que 0 para cada opción.';

  @override
  String optionsChooseTitle(String product) {
    return 'Opciones para $product';
  }

  @override
  String get optionsChooseHelp =>
      'Las opciones obligatorias se añaden automáticamente. Marque las opcionales que desee añadir.';

  @override
  String get optionsAddSelected => 'Añadir';

  @override
  String get optionsSkip => 'Sin opciones opcionales';

  @override
  String get optionLine => 'Opción de la línea anterior';

  @override
  String get configurator => 'Configurador';

  @override
  String get configurators => 'Configuradores';

  @override
  String get configure => 'Configurar';

  @override
  String get newConfiguration => 'Nueva configuración';

  @override
  String get savedConfigurations => 'Configuraciones guardadas';

  @override
  String cfgAddRow(String item) {
    return 'Añadir $item';
  }

  @override
  String cfgAddRowToGroup(String group) {
    return 'Añadir una fila a $group';
  }

  @override
  String get cfgCalculated => 'Calculado';

  @override
  String get cfgChangeCustomer => 'Otro cliente';

  @override
  String get cfgCreateQuotation => 'Crear presupuesto';

  @override
  String get cfgUpdateQuotation => 'Actualizar presupuesto';

  @override
  String get cfgDeleteConfirm =>
      '¿Eliminar esta configuración? Los presupuestos creados a partir de ella se conservan.';

  @override
  String get cfgDeleteRow => 'Eliminar fila';

  @override
  String get cfgDuplicateRow => 'Duplicar fila';

  @override
  String get cfgDiscard => 'Descartar';

  @override
  String get cfgDiscardChanges => '¿Descartar los cambios no guardados?';

  @override
  String cfgEach(String value) {
    return 'Por cada uno: $value';
  }

  @override
  String get cfgErrors => 'Errores';

  @override
  String get cfgWarnings => 'Avisos';

  @override
  String get cfgInfo => 'Información';

  @override
  String cfgEvalFailed(String message) {
    return 'No se pudo recalcular: $message';
  }

  @override
  String get cfgInput => 'Entrada';

  @override
  String get cfgIssues => 'Avisos';

  @override
  String get cfgLines => 'Líneas';

  @override
  String get cfgWhy => '¿Por qué?';

  @override
  String get cfgInvalidNumber => 'Número no válido';

  @override
  String get cfgNewerRevision =>
      'Hay una revisión más reciente de este configurador.';

  @override
  String get cfgUpgrade => 'Actualizar';

  @override
  String get cfgNoConfigurations => 'Todavía no hay configuraciones guardadas.';

  @override
  String get cfgNoModels => 'Todavía no hay ningún configurador activo.';

  @override
  String get cfgNoCustomer => 'Sin cliente';

  @override
  String get cfgNoIssues => 'Sin avisos.';

  @override
  String get cfgNoLines => 'La configuración aún no genera líneas.';

  @override
  String get cfgNotInCatalogue => 'No está en el catálogo de productos';

  @override
  String get cfgPinned => 'Valor propio (fijado)';

  @override
  String get cfgPricesNeedCustomer => 'Elija un cliente para ver los precios.';

  @override
  String get cfgQuoteBlocked =>
      'Corrija los errores antes de crear un presupuesto.';

  @override
  String get cfgQuoteNeedsCustomer =>
      'Elija un cliente antes de crear un presupuesto.';

  @override
  String cfgQuoteCreated(String number) {
    return 'Presupuesto $number creado.';
  }

  @override
  String cfgQuoteUpdated(String number) {
    return 'Presupuesto $number actualizado.';
  }

  @override
  String cfgQuoteRevised(String number) {
    return 'Presupuesto $number: se ha creado una nueva revisión.';
  }

  @override
  String cfgResetToCalculated(String value) {
    return 'Volver al valor calculado ($value)';
  }

  @override
  String cfgStale(String value) {
    return 'El cálculo ahora da $value';
  }

  @override
  String cfgRevision(int revision) {
    return 'revisión $revision';
  }

  @override
  String cfgRule(String rule) {
    return 'Regla: $rule';
  }

  @override
  String get cfgSaved => 'Guardado.';

  @override
  String get cfgSelectLine => 'Seleccione una línea para ver de dónde viene.';

  @override
  String get cfgSetForAll => 'Definir para todas las filas…';

  @override
  String cfgSetForAllTitle(String column) {
    return '$column para todas las filas';
  }

  @override
  String get cfgStart => 'Empezar';

  @override
  String get cfgUnsaved => 'Cambios sin guardar';

  @override
  String cfgNewerRevisionAvailable(int revision) {
    return 'revisión $revision disponible';
  }

  @override
  String get cfgbActivate => 'Activar';

  @override
  String get cfgbActivateConfirm =>
      '¿Activar este borrador? Ventas lo usará para nuevas configuraciones; la revisión activa queda obsoleta.';

  @override
  String get cfgbActivated => 'Activado.';

  @override
  String cfgbActivatedOn(String date) {
    return 'activado el $date';
  }

  @override
  String get cfgbActivateModel => 'Poner a disposición';

  @override
  String cfgbActivateRefused(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count problemas',
      one: '1 problema',
    );
    return 'No activado: $_temp0 por resolver.';
  }

  @override
  String cfgbActiveRevision(int revision) {
    return 'activa: revisión $revision';
  }

  @override
  String get cfgbAddChild => 'Añadir línea de opción';

  @override
  String get cfgbAddColumn => 'Añadir columna';

  @override
  String get cfgbAddComputed => 'Añadir valor calculado';

  @override
  String get cfgbAddField => 'Añadir campo';

  @override
  String get cfgbAddItemTable => 'Añadir tabla de elementos';

  @override
  String get cfgbAddOption => 'Añadir opción';

  @override
  String get cfgbAddRow => 'Añadir fila';

  @override
  String get cfgbAddRule => 'Añadir regla de salida';

  @override
  String get cfgbAddSection => 'Añadir sección';

  @override
  String get cfgbAddTable => 'Añadir tabla';

  @override
  String get cfgbAddValidation => 'Añadir comprobación';

  @override
  String get cfgbApply => 'Aplicar';

  @override
  String cfgbChildRuleTitle(String id) {
    return 'Línea de opción $id';
  }

  @override
  String get cfgbCode => 'Código de producto (fórmula)';

  @override
  String get cfgbCodeExpr => 'Fórmula';

  @override
  String get cfgbCodeHelp => 'p. ej. \'BELT-M\' o motor_code';

  @override
  String get cfgbCodeTemplate => 'Código de producto (plantilla)';

  @override
  String get cfgbCodeTemplateHelp =>
      'Texto con fórmulas entre llaves, p. ej. DRV- y luego motor_size entre llaves';

  @override
  String get cfgbColumn => 'Columna';

  @override
  String get cfgbColumnName => 'Columna';

  @override
  String get cfgbCommentTemplate => 'Comentario (plantilla)';

  @override
  String cfgbComputedOf(String scope) {
    return 'Valores calculados: $scope';
  }

  @override
  String cfgbComputedTitle(String name) {
    return 'Valor calculado $name';
  }

  @override
  String get cfgbConditions => 'Condiciones';

  @override
  String get cfgbCreate => 'Crear';

  @override
  String get cfgbCsvExport => 'Exportar CSV';

  @override
  String get cfgbCsvImport => 'Importar CSV';

  @override
  String cfgbCsvImported(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count filas importadas.',
      one: '1 fila importada.',
    );
    return '$_temp0';
  }

  @override
  String get cfgbCsvProblems => 'El archivo CSV no se importó';

  @override
  String get cfgbDeactivateModel => 'Retirar';

  @override
  String get cfgbDefault => 'Valor predeterminado';

  @override
  String get cfgbDefaultExpr => 'Valor predeterminado (fórmula)';

  @override
  String get cfgbDefaultExprHelp => 'En lugar de un valor fijo';

  @override
  String get cfgbDescriptionTemplate => 'Descripción (plantilla)';

  @override
  String get cfgbDesignTitle => 'Diseñar configuradores';

  @override
  String get cfgbDimCurrent => 'Corriente';

  @override
  String get cfgbDimension => 'Magnitud';

  @override
  String get cfgbDimLength => 'Longitud';

  @override
  String get cfgbDimMass => 'Masa';

  @override
  String get cfgbDimPower => 'Potencia';

  @override
  String get cfgbDimSpeed => 'Velocidad';

  @override
  String get cfgbDimVoltage => 'Tensión';

  @override
  String get cfgbDiscard => 'Descartar';

  @override
  String get cfgbDisplayUnits => 'Unidades de visualización:';

  @override
  String cfgbDraftRevision(int revision) {
    return 'borrador: revisión $revision';
  }

  @override
  String get cfgbEditDraft => 'Editar borrador';

  @override
  String get cfgbEnabledIf => 'Activo si';

  @override
  String cfgbExported(String name) {
    return '$name guardado.';
  }

  @override
  String get cfgbExportJson => 'Exportar JSON';

  @override
  String get cfgbExpr => 'Fórmula';

  @override
  String get cfgbFamily => 'Familia de productos (código)';

  @override
  String cfgbFieldsOf(String scope) {
    return 'Campos: $scope';
  }

  @override
  String cfgbFieldTitle(String name) {
    return 'Campo $name';
  }

  @override
  String get cfgbForEach => 'Para cada';

  @override
  String get cfgbForEachHelp =>
      'Una lista: una línea por elemento, con each y each_index';

  @override
  String cfgbFormulaType(String type) {
    return 'Resultado: $type';
  }

  @override
  String get cfgbGroupField => 'Agrupar por campo';

  @override
  String get cfgbGroupHelp =>
      'Un grupo es una sección de filas (p. ej. una nave): uno por valor del campo de agrupación de una tabla de elementos. Un campo de grupo con el nombre de un campo del proyecto lo sustituye dentro del grupo.';

  @override
  String get cfgbGroupLabel => 'Nombre de un grupo';

  @override
  String get cfgbHelp => 'Texto de ayuda';

  @override
  String get cfgbId => 'Id';

  @override
  String get cfgbImported => 'Importado en el editor; guarde para conservarlo.';

  @override
  String cfgbImportFailed(String message) {
    return 'No importado: $message';
  }

  @override
  String get cfgbImportJson => 'Importar JSON';

  @override
  String get cfgbImportNotExport =>
      'Este archivo no es una exportación de configurador.';

  @override
  String cfgbImportReplace(String name) {
    return '¿Sustituir toda la definición de este borrador por $name?';
  }

  @override
  String get cfgbInactive => 'no disponible para ventas';

  @override
  String get cfgbItemLabel => 'Etiqueta de elemento (plantilla)';

  @override
  String get cfgbItemLabelHelp =>
      'Nombra una fila en comentarios y mensajes; fórmulas entre llaves, p. ej. group_name e index';

  @override
  String get cfgbItemTable => 'Tabla de elementos';

  @override
  String get cfgbKeys => 'Columnas clave:';

  @override
  String get cfgbKindExact => 'Exacta';

  @override
  String get cfgbKindRange => 'Rango';

  @override
  String get cfgbLabel => 'Etiqueta';

  @override
  String get cfgbLayout => 'Diseño';

  @override
  String get cfgbLayoutSections => 'Secciones del formulario';

  @override
  String get cfgbLayoutSectionsHelp =>
      'Los campos indican una sección en su diseño; el formulario los muestra bajo su título.';

  @override
  String get cfgbMax => 'Máximo';

  @override
  String get cfgbMaxLength => 'Longitud máxima';

  @override
  String get cfgbMaxRows => 'Filas máximas';

  @override
  String get cfgbMergeIdentical => 'Fusionar líneas idénticas';

  @override
  String get cfgbMergeIdenticalHelp =>
      'Las líneas con el mismo código se unen; el comentario enumera los elementos';

  @override
  String get cfgbMessage => 'Mensaje (plantilla)';

  @override
  String get cfgbMin => 'Mínimo';

  @override
  String get cfgbMinRows => 'Filas mínimas';

  @override
  String get cfgbMoveDown => 'Bajar';

  @override
  String get cfgbMoveUp => 'Subir';

  @override
  String get cfgbNewDraft => 'Nuevo borrador desde la revisión activa';

  @override
  String get cfgbNewDraftFromThis => 'Nuevo borrador desde esta revisión';

  @override
  String get cfgbNewModel => 'Nuevo configurador';

  @override
  String get cfgbNoProblems => 'Sin problemas';

  @override
  String get cfgbOnMissing => 'Si ninguna fila encaja';

  @override
  String get cfgbOnMissingClamp => 'Siguiente fila superior';

  @override
  String get cfgbOnMissingHelp =>
      'Siguiente fila superior: un valor por debajo o entre los rangos toma la fila del siguiente rango superior, un valor por encima de todos los rangos la última fila; un desplazamiento más allá de un extremo se detiene en la primera o la última fila.';

  @override
  String get cfgbOnMissingError => 'Error';

  @override
  String get cfgbOnMissingNull => 'Sin valor';

  @override
  String get cfgbOptionsFamily => 'Familia de productos';

  @override
  String get cfgbOptionsFrom => 'Alternativas permitidas (fórmula)';

  @override
  String get cfgbOptionsFromHelp =>
      'Una lista entre la que el usuario puede elegir en lugar del valor calculado';

  @override
  String get cfgbOptionsStatic => 'Lista fija';

  @override
  String get cfgbOptionsTable => 'Columna de tabla';

  @override
  String get cfgbOrder => 'Orden';

  @override
  String get cfgbOutputRules => 'Reglas de salida';

  @override
  String get cfgbOverridable => 'El usuario puede sustituirlo';

  @override
  String get cfgbOverridableHelp =>
      'Se muestra como calculado, fijado o fijado pero desactualizado';

  @override
  String get cfgbPattern => 'Patrón (expresión regular)';

  @override
  String get cfgbPercentBase => 'Porcentaje de';

  @override
  String get cfgbPercentBaseList => 'el precio de lista de la línea principal';

  @override
  String get cfgbPercentBaseNet => 'el precio neto de la línea principal';

  @override
  String get cfgbPercentOfParent => 'Porcentaje del padre (fórmula)';

  @override
  String get cfgbPercentOfParentHelp =>
      'Valora la línea como este porcentaje de la línea principal, p. ej. 20';

  @override
  String cfgbProblems(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count problemas',
      one: '1 problema',
    );
    return '$_temp0';
  }

  @override
  String get cfgbProductFilter => 'Filtro de productos (fórmula sobre p)';

  @override
  String get cfgbProductFilterHelp => 'p. ej. p.attr(\'kw\') >= 1';

  @override
  String get cfgbQuantity => 'Cantidad (fórmula)';

  @override
  String get cfgbQuantityHelp => '1 por defecto; cero o menos omite la línea';

  @override
  String get cfgbRangeKey => 'Clave de rango';

  @override
  String get cfgbRangeKeyHelp => 'La clave de búsqueda con el valor';

  @override
  String get cfgbRangeMax => 'Columna máxima';

  @override
  String get cfgbRangeMin => 'Columna mínima';

  @override
  String get cfgbRawJson => 'Definición en JSON';

  @override
  String get cfgbRemoveColumn => 'Quitar columna';

  @override
  String get cfgbRemoveItemTable => 'Quitar tabla de elementos';

  @override
  String get cfgbRemoveRow => 'Quitar fila';

  @override
  String get cfgbReplace => 'Sustituir';

  @override
  String get cfgbRequired => 'Obligatorio';

  @override
  String get cfgbRevisions => 'Revisiones';

  @override
  String cfgbRowCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count filas',
      one: '1 fila',
    );
    return '$_temp0';
  }

  @override
  String cfgbRuleTitle(String id) {
    return 'Regla de salida $id';
  }

  @override
  String get cfgbSavedValid => 'Guardado. El borrador se puede activar.';

  @override
  String cfgbSavedWithProblems(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count problemas',
      one: '1 problema',
    );
    return 'Guardado con $_temp0.';
  }

  @override
  String get cfgbScope => 'Ámbito';

  @override
  String get cfgbScopeGroup => 'Grupo';

  @override
  String get cfgbScopeItem => 'Elemento (fila)';

  @override
  String cfgbScopeItems(String table) {
    return 'Elementos: $table';
  }

  @override
  String get cfgbScopeProject => 'Proyecto';

  @override
  String get cfgbScopes => 'Ámbitos';

  @override
  String get cfgbSection => 'Sección';

  @override
  String get cfgbSectionTemplate => 'Sección (plantilla)';

  @override
  String get cfgbSectionTemplateHelp =>
      'Por defecto el nombre del grupo en reglas de grupo y elemento';

  @override
  String get cfgbSelectComputed => 'Elija un valor calculado o añada uno.';

  @override
  String get cfgbSelectField => 'Elija un campo o añada uno.';

  @override
  String get cfgbSelectRule => 'Elija una regla de salida o añada una.';

  @override
  String get cfgbSelectTable => 'Elija una tabla o añada una.';

  @override
  String get cfgbSelectValidation => 'Elija una comprobación o añada una.';

  @override
  String get cfgbSeverity => 'Gravedad';

  @override
  String get cfgbSeverityError => 'Error';

  @override
  String get cfgbSeverityInfo => 'Información';

  @override
  String get cfgbSeverityWarning => 'Aviso';

  @override
  String get cfgbShow => 'Visible para el usuario';

  @override
  String get cfgbShownAt => 'Mostrado en el campo';

  @override
  String get cfgbStatusActive => 'activa';

  @override
  String get cfgbStatusDraft => 'borrador';

  @override
  String get cfgbStatusObsolete => 'obsoleta';

  @override
  String get cfgbStep => 'Paso';

  @override
  String get cfgbTabComputed => 'Valores calculados';

  @override
  String get cfgbTabFields => 'Campos y diseño';

  @override
  String get cfgbTabGeneral => 'General';

  @override
  String get cfgbTable => 'Tabla';

  @override
  String get cfgbTableKind => 'Tipo';

  @override
  String get cfgbTables => 'Tablas de búsqueda';

  @override
  String cfgbTableTitle(String name) {
    return 'Tabla $name';
  }

  @override
  String get cfgbTabOutputs => 'Reglas de salida';

  @override
  String get cfgbTabTables => 'Tablas';

  @override
  String get cfgbTabTest => 'Probar';

  @override
  String get cfgbTabTransfer => 'Importar/exportar';

  @override
  String get cfgbTabValidations => 'Comprobaciones';

  @override
  String get cfgbTestHelp =>
      'Pruebe el borrador guardado tal como lo verá ventas. No se guarda nada; elija una línea para ver por qué está.';

  @override
  String get cfgbTestNeedsValid =>
      'El borrador guardado tiene problemas; la prueba puede fallar hasta resolverlos.';

  @override
  String get cfgbTestReset => 'Empezar de nuevo';

  @override
  String get cfgbTestUnsaved =>
      'La prueba usa el borrador guardado; guarde para incluir sus últimos cambios.';

  @override
  String get cfgbTranslations => 'Traducciones';

  @override
  String get cfgbTypeBoolean => 'Sí/no';

  @override
  String get cfgbTypeChoice => 'Elección';

  @override
  String get cfgbTypeInteger => 'Número entero';

  @override
  String get cfgbTypeNumber => 'Número';

  @override
  String get cfgbTypeProduct => 'Producto';

  @override
  String get cfgbTypeText => 'Texto';

  @override
  String get cfgbUnsavedText => 'El borrador tiene cambios sin guardar.';

  @override
  String get cfgbUnsavedTitle => 'Cambios sin guardar';

  @override
  String get cfgbValidations => 'Comprobaciones';

  @override
  String cfgbValidationTitle(String id) {
    return 'Comprobación $id';
  }

  @override
  String get cfgbValidationWhenHelp => 'Si es verdadero, se muestra el mensaje';

  @override
  String get cfgbValue => 'Valor';

  @override
  String get cfgbVisibleIf => 'Visible si';

  @override
  String get cfgbWhen => 'Cuando';

  @override
  String get cfgbWhenHelp => 'Las líneas solo se crean si esto es verdadero';

  @override
  String get cfgbWidth => 'Anchura';

  @override
  String get tabProductionOrder => 'Orden de producción';

  @override
  String get tabDeliveryNote => 'Albarán';

  @override
  String get vatRegime => 'Régimen de IVA';

  @override
  String get vatRegimeDomestic => 'Cliente UE, con IVA';

  @override
  String get vatRegimeIntraEu => 'Empresa UE, inversión del sujeto pasivo';

  @override
  String get vatRegimeOutsideEu => 'Fuera de la UE, sin IVA';

  @override
  String get addSubItem => 'Añadir subelemento';

  @override
  String subItemsOptions(String product) {
    return 'Opciones de $product';
  }

  @override
  String get subItemsNoOptions => 'Este producto no tiene opciones.';

  @override
  String get subItemsOtherProducts => 'Otros productos';

  @override
  String get crTitle => 'Informes';

  @override
  String get crNew => 'Nuevo informe';

  @override
  String get crEmpty =>
      'Aún no hay informes. Cree uno para agrupar y sumar sus datos.';

  @override
  String get crShared => 'Compartido';

  @override
  String get crPrivate => 'Privado';

  @override
  String get crShare => 'Compartir con toda la empresa';

  @override
  String crOwnedBy(String name) {
    return 'Por $name';
  }

  @override
  String get crObject => 'Informe sobre';

  @override
  String get crJoin => 'Añadir datos relacionados';

  @override
  String get crGroupBy => 'Agrupar por';

  @override
  String get crAddGrouping => 'Añadir agrupación';

  @override
  String get crColumns => 'Columnas';

  @override
  String get crAddColumn => 'Añadir columna';

  @override
  String get crField => 'Campo';

  @override
  String get crSummary => 'Resumen';

  @override
  String get crSummaryNone => 'Mostrar valor';

  @override
  String get crSumCount => 'Recuento';

  @override
  String get crSumSum => 'Suma';

  @override
  String get crSumAvg => 'Promedio';

  @override
  String get crSumMin => 'Mínimo';

  @override
  String get crSumMax => 'Máximo';

  @override
  String get crFilters => 'Filtros';

  @override
  String get crAddFilter => 'Añadir filtro';

  @override
  String get crMatchAll => 'Deben cumplirse todos los filtros';

  @override
  String get crMatchAny => 'Debe cumplirse al menos un filtro';

  @override
  String get crOpEq => 'es igual a';

  @override
  String get crOpNe => 'no es igual a';

  @override
  String get crOpGt => 'mayor que';

  @override
  String get crOpGte => 'al menos';

  @override
  String get crOpLt => 'menor que';

  @override
  String get crOpLte => 'como máximo';

  @override
  String get crOpContains => 'contiene';

  @override
  String get crOpStartsWith => 'empieza por';

  @override
  String get crOpIn => 'es uno de';

  @override
  String get crOpIsNull => 'está vacío';

  @override
  String get crOpNotNull => 'no está vacío';

  @override
  String get crValue => 'Valor';

  @override
  String get crValueListHint => 'Separe los valores con comas';

  @override
  String get crBucketNone => 'Valor exacto';

  @override
  String get crBucketDay => 'Por día';

  @override
  String get crBucketMonth => 'Por mes';

  @override
  String get crBucketYear => 'Por año';

  @override
  String get crSort => 'Ordenar';

  @override
  String get crAddSort => 'Añadir orden';

  @override
  String get crDescending => 'Descendente';

  @override
  String get crChart => 'Gráfico';

  @override
  String get crChartNone => 'Sin gráfico';

  @override
  String get crChartBar => 'Barras';

  @override
  String get crChartLine => 'Línea';

  @override
  String get crChartPie => 'Circular';

  @override
  String get crChartX => 'Categorías';

  @override
  String get crChartY => 'Valores';

  @override
  String crRows(int count) {
    return '$count filas';
  }

  @override
  String crRowsShown(int shown, int total) {
    return '$shown de $total filas mostradas';
  }

  @override
  String crChartShareShown(int count) {
    return 'Los porcentajes son de los $count grupos mostrados.';
  }

  @override
  String get crSaved => 'Informe guardado';

  @override
  String get crDeleted => 'Informe eliminado';

  @override
  String get crDelete => 'Eliminar';

  @override
  String get crDeleteConfirm => '¿Eliminar este informe?';

  @override
  String get crExportCsv => 'Exportar CSV';

  @override
  String get crExportPdf => 'Exportar PDF';

  @override
  String get crNeedName => 'Dé un nombre al informe.';

  @override
  String get crNeedColumn => 'Añada al menos una columna.';

  @override
  String get crLoadMore => 'Mostrar más filas';

  @override
  String get crNoRows => 'Ningún resultado para este informe.';

  @override
  String get crRun => 'Ejecutar';

  @override
  String get crDateMode => 'Fecha';

  @override
  String get crDateFixed => 'Fecha fija';

  @override
  String get crDateRelative => 'Relativa a hoy';

  @override
  String get crRelativePeriod => 'Periodo';

  @override
  String get crRelativeDays => 'Número de días';

  @override
  String get crRelativeDaysInvalid =>
      'Introduce un número entero de días entre 1 y 3660.';

  @override
  String get crRelToday => 'Hoy';

  @override
  String get crRelYesterday => 'Ayer';

  @override
  String get crRelThisWeek => 'Esta semana';

  @override
  String get crRelLastWeek => 'Semana pasada';

  @override
  String get crRelThisMonth => 'Este mes';

  @override
  String get crRelLastMonth => 'Mes pasado';

  @override
  String get crRelThisQuarter => 'Este trimestre';

  @override
  String get crRelLastQuarter => 'Trimestre pasado';

  @override
  String get crRelThisYear => 'Este año';

  @override
  String get crRelLastYear => 'Año pasado';

  @override
  String get crRelLastNDays => 'Últimos N días';

  @override
  String get crRelNextNDays => 'Próximos N días';

  @override
  String get adminPriceZones => 'Zonas de precio';

  @override
  String get newPriceZone => 'Nueva zona de precio';

  @override
  String get editPriceZone => 'Editar zona de precio';

  @override
  String get sortOrder => 'Orden';

  @override
  String get noPriceZones => 'Aún no hay zonas de precio.';

  @override
  String get priceZoneDeleted => 'Zona de precio eliminada';

  @override
  String get delete => 'Eliminar';

  @override
  String deletePriceZoneConfirm(String code) {
    return '¿Eliminar la zona de precio $code? Se rechaza si hay precios o clientes que la usan.';
  }
}
